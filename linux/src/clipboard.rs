use anyhow::{bail, Context, Result};
use desklink_protocol::{
    decode_control, encode_control, ControlMessage, MAX_CONTROL_HANDSHAKE_BYTES, PROTOCOL_VERSION,
};
use std::{process::Stdio, time::Duration};
use tokio::{
    io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt},
    net::{TcpListener, TcpStream},
    process::Command,
    time,
};
use tracing::{info, warn};

async fn write_message(
    writer: &mut (impl AsyncWrite + Unpin),
    message: &ControlMessage,
) -> Result<()> {
    let data = encode_control(message)?;
    if data.len() > u32::MAX as usize {
        bail!("control message is too large for the wire format");
    }
    writer.write_u32(data.len() as u32).await?;
    writer.write_all(&data).await?;
    writer.flush().await?;
    Ok(())
}

async fn read_message(
    reader: &mut (impl AsyncRead + Unpin),
    max_frame_bytes: usize,
) -> Result<ControlMessage> {
    let length = reader.read_u32().await? as usize;
    if length > max_frame_bytes {
        bail!("control frame exceeds configured size limit");
    }
    let mut data = vec![0; length];
    reader.read_exact(&mut data).await?;
    Ok(decode_control(&data)?)
}

async fn read_text() -> Result<Option<String>> {
    let output = Command::new("wl-paste")
        .args(["--no-newline", "--type", "text"])
        .output()
        .await
        .context("run wl-paste")?;
    if !output.status.success() {
        return Ok(None);
    }
    Ok(Some(
        String::from_utf8(output.stdout).context("clipboard is not UTF-8 text")?,
    ))
}

async fn write_text(text: &str) -> Result<()> {
    let mut child = Command::new("wl-copy")
        .arg("--type")
        .arg("text/plain;charset=utf-8")
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .context("run wl-copy")?;
    let mut stdin = child.stdin.take().context("open wl-copy stdin")?;
    stdin.write_all(text.as_bytes()).await?;
    stdin.shutdown().await?;
    drop(stdin);
    let status = child.wait().await?;
    if !status.success() {
        bail!("wl-copy exited with {status}");
    }
    Ok(())
}

async fn handle_client(
    stream: TcpStream,
    token: String,
    poll_ms: u64,
    max_bytes: usize,
) -> Result<()> {
    let peer = stream.peer_addr()?;
    let (mut reader, mut writer) = stream.into_split();
    match read_message(&mut reader, MAX_CONTROL_HANDSHAKE_BYTES).await? {
        ControlMessage::Hello { version, .. } if version != PROTOCOL_VERSION => {
            write_message(
                &mut writer,
                &ControlMessage::Reject {
                    reason: format!(
                        "protocol mismatch: windows={version}, linux={PROTOCOL_VERSION}"
                    ),
                },
            )
            .await?;
            bail!("clipboard protocol mismatch");
        }
        ControlMessage::Hello {
            token: supplied, ..
        } if supplied != token => {
            write_message(
                &mut writer,
                &ControlMessage::Reject {
                    reason: "shared token mismatch".to_owned(),
                },
            )
            .await?;
            bail!("clipboard token mismatch");
        }
        ControlMessage::Hello { .. } => write_message(&mut writer, &ControlMessage::Ready).await?,
        _ => bail!("expected clipboard handshake"),
    }

    info!(?peer, "clipboard channel connected");
    let mut last_text = read_text().await.ok().flatten();
    let frame_limit = max_bytes
        .saturating_add(MAX_CONTROL_HANDSHAKE_BYTES)
        .min(u32::MAX as usize);
    let mut next_id = 1u64;
    let mut poll = time::interval(Duration::from_millis(poll_ms.clamp(100, 10_000)));
    poll.set_missed_tick_behavior(time::MissedTickBehavior::Skip);
    loop {
        tokio::select! {
            _ = poll.tick() => {
                let Ok(current) = read_text().await else { continue };
                if current != last_text {
                    last_text = current.clone();
                    if let Some(text) = current {
                        if text.len() > max_bytes {
                            warn!(bytes = text.len(), max_bytes, "clipboard text is larger than configured limit; skipped");
                            continue;
                        }
                        write_message(&mut writer, &ControlMessage::ClipboardText { id: next_id, text }).await?;
                        next_id = next_id.wrapping_add(1);
                    }
                }
            }
            message = read_message(&mut reader, frame_limit) => {
                match message? {
                    ControlMessage::ClipboardText { text, .. } => {
                        if text.len() > max_bytes {
                            warn!(bytes = text.len(), max_bytes, "remote clipboard text is larger than configured limit; skipped");
                        } else if last_text.as_deref() != Some(text.as_str()) {
                            write_text(&text).await?;
                            last_text = Some(text);
                            info!("clipboard text received from Windows");
                        }
                    }
                    ControlMessage::Reject { reason } => bail!("clipboard channel rejected: {reason}"),
                    _ => {}
                }
            }
        }
    }
}

pub async fn run(bind: String, token: String, poll_ms: u64, max_bytes: usize) -> Result<()> {
    Command::new("wl-paste")
        .arg("--version")
        .output()
        .await
        .context("wl-paste is unavailable; install the wl-clipboard package")?;
    Command::new("wl-copy")
        .arg("--version")
        .output()
        .await
        .context("wl-copy is unavailable; install the wl-clipboard package")?;
    let listener = TcpListener::bind(&bind)
        .await
        .context("bind clipboard control port")?;
    info!(%bind, max_bytes, "clipboard service listening");
    loop {
        let (stream, peer) = listener.accept().await?;
        let token = token.clone();
        tokio::spawn(async move {
            if let Err(error) = handle_client(stream, token, poll_ms, max_bytes).await {
                warn!(%error, ?peer, "clipboard client disconnected");
            }
        });
    }
}

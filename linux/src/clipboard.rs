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

#[derive(Debug, Clone, Copy)]
enum ClipboardBackend {
    Wayland,
    X11,
}

impl ClipboardBackend {
    fn name(self) -> &'static str {
        match self {
            Self::Wayland => "wayland/wl-clipboard",
            Self::X11 => "x11/xclip",
        }
    }
}

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

async fn read_text(backend: ClipboardBackend) -> Result<Option<String>> {
    let output = match backend {
        ClipboardBackend::Wayland => Command::new("wl-paste")
            .args(["--no-newline", "--type", "text"])
            .output()
            .await
            .context("run wl-paste")?,
        ClipboardBackend::X11 => Command::new("xclip")
            .args(["-selection", "clipboard", "-out"])
            .output()
            .await
            .context("run xclip")?,
    };
    if !output.status.success() {
        let detail = String::from_utf8_lossy(&output.stderr).trim().to_owned();
        if detail.is_empty()
            || detail.contains("No selection")
            || detail.contains("doesn't offer text")
        {
            return Ok(None);
        }
        bail!("{} clipboard read failed: {detail}", backend.name());
    }
    Ok(Some(
        String::from_utf8(output.stdout).context("clipboard is not UTF-8 text")?,
    ))
}

async fn write_text(backend: ClipboardBackend, text: &str) -> Result<()> {
    let mut command = match backend {
        ClipboardBackend::Wayland => {
            let mut command = Command::new("wl-copy");
            command.arg("--type").arg("text/plain;charset=utf-8");
            command
        }
        ClipboardBackend::X11 => {
            let mut command = Command::new("xclip");
            command.args(["-selection", "clipboard", "-in"]);
            command
        }
    };
    let mut child = command
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        // wl-copy and xclip can fork a long-lived clipboard owner. A piped
        // stderr would remain open in that process and make wait_with_output
        // block until the selection is replaced.
        .stderr(Stdio::inherit())
        .spawn()
        .with_context(|| format!("start {} clipboard writer", backend.name()))?;
    let mut stdin = child.stdin.take().context("open clipboard writer stdin")?;
    stdin.write_all(text.as_bytes()).await?;
    stdin.shutdown().await?;
    drop(stdin);
    let status = child.wait().await?;
    if !status.success() {
        bail!("{} clipboard write failed ({})", backend.name(), status);
    }
    Ok(())
}

async fn handle_client(
    stream: TcpStream,
    token: String,
    poll_ms: u64,
    max_bytes: usize,
    backend: ClipboardBackend,
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

    info!(
        ?peer,
        backend = backend.name(),
        "clipboard channel connected"
    );
    let (mut last_text, mut last_read_error) = match read_text(backend).await {
        Ok(text) => (text, None),
        Err(error) => {
            let detail = error.to_string();
            warn!(%error, backend = backend.name(), "cannot read initial Linux clipboard; channel remains connected");
            (None, Some(detail))
        }
    };
    let frame_limit = max_bytes
        .saturating_add(MAX_CONTROL_HANDSHAKE_BYTES)
        .min(u32::MAX as usize);
    let mut next_id = 1u64;
    let mut poll = time::interval(Duration::from_millis(poll_ms.clamp(100, 10_000)));
    poll.set_missed_tick_behavior(time::MissedTickBehavior::Skip);
    loop {
        tokio::select! {
            _ = poll.tick() => {
                let current = match read_text(backend).await {
                    Ok(current) => {
                        last_read_error = None;
                        current
                    }
                    Err(error) => {
                        let detail = error.to_string();
                        if last_read_error.as_deref() != Some(detail.as_str()) {
                            warn!(%error, backend = backend.name(), "cannot read Linux clipboard; channel remains connected");
                            last_read_error = Some(detail);
                        }
                        continue;
                    }
                };
                if current != last_text {
                    last_text = current.clone();
                    if let Some(text) = current {
                        if text.len() > max_bytes {
                            warn!(bytes = text.len(), max_bytes, "clipboard text is larger than configured limit; skipped");
                            continue;
                        }
                        write_message(&mut writer, &ControlMessage::ClipboardText { id: next_id, text }).await?;
                        info!(id = next_id, "clipboard text sent to Windows");
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
                            match write_text(backend, &text).await {
                                Ok(()) => {
                                    last_text = Some(text);
                                    info!(backend = backend.name(), "clipboard text received from Windows");
                                }
                                Err(error) => {
                                    warn!(%error, backend = backend.name(), "cannot write Linux clipboard; channel remains connected");
                                }
                            }
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
    let wayland_session = std::env::var_os("WAYLAND_DISPLAY").is_some();
    let x11_session = std::env::var_os("DISPLAY").is_some();
    let wl_available = Command::new("wl-paste")
        .arg("--version")
        .output()
        .await
        .is_ok()
        && Command::new("wl-copy")
            .arg("--version")
            .output()
            .await
            .is_ok();
    let xclip_available = Command::new("xclip").arg("-version").output().await.is_ok();
    let backend = if wayland_session && wl_available {
        ClipboardBackend::Wayland
    } else if x11_session && xclip_available {
        ClipboardBackend::X11
    } else if wl_available {
        ClipboardBackend::Wayland
    } else if xclip_available {
        ClipboardBackend::X11
    } else {
        bail!("no Linux clipboard backend found; install wl-clipboard (Wayland) or xclip (X11)");
    };
    let listener = TcpListener::bind(&bind)
        .await
        .context("bind clipboard control port")?;
    info!(%bind, max_bytes, backend = backend.name(), "clipboard service listening");
    loop {
        let (stream, peer) = listener.accept().await?;
        let token = token.clone();
        tokio::spawn(async move {
            if let Err(error) = handle_client(stream, token, poll_ms, max_bytes, backend).await {
                warn!(%error, ?peer, "clipboard client disconnected");
            }
        });
    }
}

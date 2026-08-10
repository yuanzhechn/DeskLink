use anyhow::{bail, Result};
use desklink_protocol::{
    decode_control, encode_control, ControlMessage, MAX_CONTROL_HANDSHAKE_BYTES, PROTOCOL_VERSION,
};
use std::{net::SocketAddr, slice, time::Duration};
use tokio::{
    io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt},
    net::TcpStream,
    time,
};
use tracing::{info, warn};
use windows_sys::Win32::{
    Foundation::GlobalFree,
    System::{
        DataExchange::{
            CloseClipboard, EmptyClipboard, GetClipboardData, OpenClipboard, SetClipboardData,
        },
        Memory::{GlobalAlloc, GlobalLock, GlobalSize, GlobalUnlock, GMEM_MOVEABLE},
        Ole::CF_UNICODETEXT,
    },
};

struct ClipboardGuard;

impl Drop for ClipboardGuard {
    fn drop(&mut self) {
        unsafe { CloseClipboard() };
    }
}

fn open_clipboard() -> Result<ClipboardGuard> {
    if unsafe { OpenClipboard(0) } == 0 {
        bail!("Windows clipboard is busy");
    }
    Ok(ClipboardGuard)
}

fn read_text() -> Result<Option<String>> {
    let _guard = open_clipboard()?;
    let handle = unsafe { GetClipboardData(CF_UNICODETEXT as u32) };
    if handle == 0 {
        return Ok(None);
    }
    let global = handle as *mut std::ffi::c_void;
    let pointer = unsafe { GlobalLock(global) } as *const u16;
    if pointer.is_null() {
        bail!("failed to lock Windows clipboard data");
    }
    let units = unsafe { GlobalSize(global) } / size_of::<u16>();
    let data = unsafe { slice::from_raw_parts(pointer, units) };
    let length = data.iter().position(|unit| *unit == 0).unwrap_or(units);
    let text = String::from_utf16_lossy(&data[..length]);
    unsafe { GlobalUnlock(global) };
    Ok(Some(text))
}

fn write_text(text: &str) -> Result<()> {
    let utf16: Vec<u16> = text.encode_utf16().chain(std::iter::once(0)).collect();
    let bytes = utf16.len() * size_of::<u16>();
    let memory = unsafe { GlobalAlloc(GMEM_MOVEABLE, bytes) };
    if memory.is_null() {
        bail!("failed to allocate Windows clipboard data");
    }
    let pointer = unsafe { GlobalLock(memory) } as *mut u16;
    if pointer.is_null() {
        unsafe { GlobalFree(memory) };
        bail!("failed to lock new Windows clipboard data");
    }
    unsafe {
        pointer.copy_from_nonoverlapping(utf16.as_ptr(), utf16.len());
        GlobalUnlock(memory);
    }
    let guard = match open_clipboard() {
        Ok(guard) => guard,
        Err(error) => {
            unsafe { GlobalFree(memory) };
            return Err(error);
        }
    };
    if unsafe { EmptyClipboard() } == 0
        || unsafe { SetClipboardData(CF_UNICODETEXT as u32, memory as isize) } == 0
    {
        drop(guard);
        unsafe { GlobalFree(memory) };
        bail!("failed to update Windows clipboard");
    }
    // SetClipboardData transfers ownership of memory to Windows.
    Ok(())
}

async fn write_text_with_retry(text: &str) -> Result<()> {
    let mut last_error = None;
    for _ in 0..5 {
        match write_text(text) {
            Ok(()) => return Ok(()),
            Err(error) => last_error = Some(error),
        }
        time::sleep(Duration::from_millis(50)).await;
    }
    Err(last_error.expect("clipboard retry loop always runs"))
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

async fn connected_session(
    stream: TcpStream,
    token: &str,
    poll_ms: u64,
    max_bytes: usize,
) -> Result<()> {
    let (mut reader, mut writer) = stream.into_split();
    write_message(
        &mut writer,
        &ControlMessage::Hello {
            version: PROTOCOL_VERSION,
            device_id: "desklink-windows".to_owned(),
            token: token.to_owned(),
        },
    )
    .await?;
    match read_message(&mut reader, MAX_CONTROL_HANDSHAKE_BYTES).await? {
        ControlMessage::Ready => {}
        ControlMessage::Reject { reason } => bail!("Linux rejected clipboard channel: {reason}"),
        _ => bail!("unexpected clipboard handshake response"),
    }

    info!("clipboard channel connected");
    let mut last_text = read_text().ok().flatten();
    let frame_limit = max_bytes
        .saturating_add(MAX_CONTROL_HANDSHAKE_BYTES)
        .min(u32::MAX as usize);
    let mut next_id = 1u64;
    if let Some(text) = last_text.as_ref() {
        if text.len() <= max_bytes {
            write_message(
                &mut writer,
                &ControlMessage::ClipboardText {
                    id: next_id,
                    text: text.clone(),
                },
            )
            .await?;
            info!(id = next_id, "initial clipboard text sent to Linux");
            next_id = next_id.wrapping_add(1);
        } else {
            warn!(
                bytes = text.len(),
                max_bytes, "initial clipboard text is larger than configured limit; skipped"
            );
        }
    }
    let mut poll = time::interval(Duration::from_millis(poll_ms.clamp(100, 10_000)));
    poll.set_missed_tick_behavior(time::MissedTickBehavior::Skip);
    loop {
        tokio::select! {
            _ = poll.tick() => {
                let Ok(current) = read_text() else { continue };
                if current != last_text {
                    last_text = current.clone();
                    if let Some(text) = current {
                        if text.len() > max_bytes {
                            warn!(bytes = text.len(), max_bytes, "clipboard text is larger than configured limit; skipped");
                            continue;
                        }
                        write_message(&mut writer, &ControlMessage::ClipboardText { id: next_id, text }).await?;
                        info!(id = next_id, "clipboard text sent to Linux");
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
                            match write_text_with_retry(&text).await {
                                Ok(()) => {
                                    last_text = Some(text);
                                    info!("clipboard text received from Linux");
                                }
                                Err(error) => {
                                    warn!(%error, "cannot write Windows clipboard; channel remains connected");
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

pub async fn run(target: SocketAddr, token: String, poll_ms: u64, max_bytes: usize) {
    loop {
        match TcpStream::connect(target).await {
            Ok(stream) => {
                if let Err(error) = connected_session(stream, &token, poll_ms, max_bytes).await {
                    warn!(%error, "clipboard channel disconnected; retrying");
                }
            }
            Err(error) => warn!(%error, %target, "clipboard channel unavailable; retrying"),
        }
        time::sleep(Duration::from_secs(2)).await;
    }
}

use anyhow::{Context, Result};
use desklink_protocol::{encode, InputEvent, Packet, DEFAULT_INPUT_PORT};
use std::{net::SocketAddr, time::{Duration, SystemTime, UNIX_EPOCH}};
use tokio::{io::{self, AsyncBufReadExt}, net::UdpSocket, time};
use tokio::sync::mpsc::{self, UnboundedReceiver, UnboundedSender};
use tracing::{info, warn};
use std::sync::{atomic::{AtomicBool, Ordering}, OnceLock};
use windows_sys::Win32::{Foundation::{LPARAM, LRESULT, WPARAM}, UI::{Input::KeyboardAndMouse::{KBDLLHOOKSTRUCT, MSLLHOOKSTRUCT}, WindowsAndMessaging::*}};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ControlState { Local, Remote, Disconnected }

static HOOK_EVENTS: OnceLock<UnboundedSender<InputEvent>> = OnceLock::new();
static REMOTE_INPUT: AtomicBool = AtomicBool::new(false);
static LAST_MOUSE_X: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);
static LAST_MOUSE_Y: std::sync::atomic::AtomicI32 = std::sync::atomic::AtomicI32::new(0);
static HAVE_MOUSE_POINT: AtomicBool = AtomicBool::new(false);
static CTRL_DOWN: AtomicBool = AtomicBool::new(false);
static ALT_DOWN: AtomicBool = AtomicBool::new(false);
static EMERGENCY_STOP: AtomicBool = AtomicBool::new(false);

fn set_remote_input(enabled: bool) { REMOTE_INPUT.store(enabled, Ordering::Release); }

fn windows_key_to_linux(vk: u32, scan_code: u32) -> u16 {
    const LETTERS: [u16; 26] = [30, 48, 46, 32, 18, 33, 34, 35, 23, 36, 37, 38, 50, 49, 24, 25, 16, 19, 31, 20, 22, 47, 17, 45, 21, 44];
    const DIGITS: [u16; 10] = [11, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    match vk {
        0x41..=0x5A => LETTERS[(vk - 0x41) as usize],
        0x30..=0x39 => DIGITS[(vk - 0x30) as usize],
        0x1B => 1, 0x08 => 14, 0x09 => 15, 0x0D => 28, 0x20 => 57,
        0xA2 | 0xA3 => 29, 0xA0 | 0xA1 => 42, 0xA4 | 0xA5 => 56,
        0x5B | 0x5C => 125, 0x25 => 105, 0x26 => 103, 0x27 => 106, 0x28 => 108,
        _ => scan_code as u16,
    }
}

unsafe extern "system" fn mouse_hook(code: i32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    if code == HC_ACTION && REMOTE_INPUT.load(Ordering::Acquire) {
        let data = &*(lparam as *const MSLLHOOKSTRUCT);
        let event = match wparam as u32 {
            WM_MOUSEMOVE => {
                let x = data.pt.x;
                let y = data.pt.y;
                let old_x = LAST_MOUSE_X.swap(x, Ordering::AcqRel);
                let old_y = LAST_MOUSE_Y.swap(y, Ordering::AcqRel);
                if HAVE_MOUSE_POINT.swap(true, Ordering::AcqRel) {
                    let dx = (x - old_x).clamp(i16::MIN as i32, i16::MAX as i32) as i16;
                    let dy = (y - old_y).clamp(i16::MIN as i32, i16::MAX as i32) as i16;
                    if dx != 0 || dy != 0 { Some(InputEvent::MouseMove { dx, dy }) } else { None }
                } else { None }
            }
            WM_LBUTTONDOWN => Some(InputEvent::MouseButton { button: 1, pressed: true }),
            WM_LBUTTONUP => Some(InputEvent::MouseButton { button: 1, pressed: false }),
            WM_RBUTTONDOWN => Some(InputEvent::MouseButton { button: 2, pressed: true }),
            WM_RBUTTONUP => Some(InputEvent::MouseButton { button: 2, pressed: false }),
            WM_MBUTTONDOWN => Some(InputEvent::MouseButton { button: 3, pressed: true }),
            WM_MBUTTONUP => Some(InputEvent::MouseButton { button: 3, pressed: false }),
            WM_MOUSEWHEEL => Some(InputEvent::MouseWheel { vertical: ((data.mouseData >> 16) as i16) / 120, horizontal: 0 }),
            WM_MOUSEHWHEEL => Some(InputEvent::MouseWheel { vertical: 0, horizontal: ((data.mouseData >> 16) as i16) / 120 }),
            _ => None,
        };
        if let Some(event) = event { if let Some(tx) = HOOK_EVENTS.get() { let _ = tx.send(event); } return 1; }
    }
    CallNextHookEx(0, code, wparam, lparam)
}

unsafe extern "system" fn keyboard_hook(code: i32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    if code == HC_ACTION && REMOTE_INPUT.load(Ordering::Acquire) {
        let data = &*(lparam as *const KBDLLHOOKSTRUCT);
        let msg = wparam as u32;
        let pressed = matches!(msg, WM_KEYDOWN | WM_SYSKEYDOWN);
        let released = matches!(msg, WM_KEYUP | WM_SYSKEYUP);
        let is_ctrl = matches!(data.vkCode, 0xA2 | 0xA3);
        let is_alt = matches!(data.vkCode, 0xA4 | 0xA5);
        if is_ctrl { CTRL_DOWN.store(pressed, Ordering::Release); }
        if is_alt { ALT_DOWN.store(pressed, Ordering::Release); }
        if pressed && data.vkCode == 0x1B && CTRL_DOWN.load(Ordering::Acquire) && ALT_DOWN.load(Ordering::Acquire) {
            REMOTE_INPUT.store(false, Ordering::Release);
            EMERGENCY_STOP.store(true, Ordering::Release);
            return 1;
        }
        if pressed || released {
            if let Some(tx) = HOOK_EVENTS.get() { let _ = tx.send(InputEvent::Key { code: windows_key_to_linux(data.vkCode, data.scanCode), pressed }); }
            return 1;
        }
    }
    CallNextHookEx(0, code, wparam, lparam)
}

fn start_windows_hooks(tx: UnboundedSender<InputEvent>) {
    let _ = HOOK_EVENTS.set(tx);
    std::thread::spawn(|| unsafe {
        let mouse = SetWindowsHookExW(WH_MOUSE_LL, Some(mouse_hook), 0, 0);
        let keyboard = SetWindowsHookExW(WH_KEYBOARD_LL, Some(keyboard_hook), 0, 0);
        if mouse == 0 || keyboard == 0 { warn!("failed to install Windows input hooks"); return; }
        let mut msg = std::mem::zeroed::<MSG>();
        while GetMessageW(&mut msg, 0, 0, 0) > 0 { TranslateMessage(&msg); DispatchMessageW(&msg); }
        UnhookWindowsHookEx(mouse);
        UnhookWindowsHookEx(keyboard);
    });
}

struct InputSender { socket: UdpSocket, target: SocketAddr, session: u64, sequence: u32 }
impl InputSender {
    async fn send(&mut self, event: InputEvent) -> Result<()> { self.sequence = self.sequence.wrapping_add(1); let p = Packet::Input { session: self.session, sequence: self.sequence, event }; self.socket.send_to(&encode(&p)?, self.target).await?; Ok(()) }
    async fn state(&self, remote: bool) -> Result<()> { let p = Packet::SetState { session: self.session, remote }; self.socket.send_to(&encode(&p)?, self.target).await?; Ok(()) }
}
fn now_ms() -> u64 { SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_millis() as u64 }

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt().with_env_filter("info").init();
    let target: SocketAddr = std::env::var("DESKLINK_TARGET").unwrap_or_else(|_| format!("127.0.0.1:{DEFAULT_INPUT_PORT}")).parse().context("invalid DESKLINK_TARGET")?;
    let socket = UdpSocket::bind("0.0.0.0:0").await?;
    let mut sender = InputSender { socket, target, session: now_ms(), sequence: 0 };
    let mut state = ControlState::Local;
    let (hook_tx, mut hook_rx): (UnboundedSender<InputEvent>, UnboundedReceiver<InputEvent>) = mpsc::unbounded_channel();
    start_windows_hooks(hook_tx);
    info!(?target, ?state, "DeskLink Windows Host started (type 'help' for commands)");
    let mut lines = io::BufReader::new(io::stdin()).lines();
    let mut heartbeat = time::interval(Duration::from_secs(2));
    loop {
        tokio::select! {
            _ = heartbeat.tick() => {
                if EMERGENCY_STOP.swap(false, Ordering::AcqRel) {
                    set_remote_input(false);
                    sender.state(false).await?;
                    let release = Packet::ReleaseAll { session: sender.session };
                    sender.socket.send_to(&encode(&release)?, sender.target).await?;
                    state = ControlState::Local;
                    warn!("emergency hotkey activated; local input restored");
                }
                let p = Packet::Heartbeat { session: sender.session, timestamp_ms: now_ms() };
                if let Err(e) = sender.socket.send_to(&encode(&p)?, sender.target).await {
                    state = ControlState::Disconnected;
                    warn!(%e, ?state, "heartbeat failed");
                } else if state == ControlState::Remote {
                    // Resend the state so a Linux client may start later or restart.
                    // 重复发送状态，允许 Linux Client 晚于 Windows 启动或重启。
                    sender.state(true).await?;
                }
            }
            Some(event) = hook_rx.recv() => { if state == ControlState::Remote { sender.send(event).await?; } }
            line = lines.next_line() => {
                let Some(line) = line? else { break };
                let mut p = line.split_whitespace();
                match p.next().unwrap_or("") {
                    "move" => { let dx: i16 = p.next().unwrap_or("0").parse()?; let dy: i16 = p.next().unwrap_or("0").parse()?; sender.send(InputEvent::MouseMove { dx, dy }).await?; }
                    "click" => { let button: u8 = p.next().unwrap_or("1").parse()?; sender.send(InputEvent::MouseButton { button, pressed: true }).await?; sender.send(InputEvent::MouseButton { button, pressed: false }).await?; }
                    "key" => { let code: u16 = p.next().unwrap_or("30").parse()?; let pressed = p.next().unwrap_or("down") != "up"; sender.send(InputEvent::Key { code, pressed }).await?; }
                    "remote" => { sender.state(true).await?; set_remote_input(true); state = ControlState::Remote; info!(?state, "remote input enabled"); }
                    "local" | "release" => { set_remote_input(false); sender.state(false).await?; let p = Packet::ReleaseAll { session: sender.session }; sender.socket.send_to(&encode(&p)?, sender.target).await?; state = ControlState::Local; info!(?state, "local input restored"); }
                    "help" => println!("move DX DY | click BUTTON | key CODE down/up | remote | local | release | quit"),
                    "quit" => break,
                    _ => println!("unknown command; type help"),
                }
            }
        }
    }
    Ok(())
}

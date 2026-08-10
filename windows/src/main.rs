mod layout;
mod web_ui;

use anyhow::{Context, Result};
use desklink_config::DeskLinkConfig;
use desklink_protocol::{decode, encode, InputEvent, Packet, ScreenEdge, PROTOCOL_VERSION};
use layout::LayoutSnapshot;
use std::{
    net::SocketAddr,
    path::PathBuf,
    sync::{
        atomic::{AtomicBool, AtomicI32, Ordering},
        Arc, OnceLock,
    },
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};
use tokio::{
    io::{self, AsyncBufReadExt},
    net::UdpSocket,
    sync::{
        mpsc::{self, Receiver, Sender},
        RwLock,
    },
    time::{self, MissedTickBehavior},
};
use tracing::{info, warn};
use web_ui::UiState;
use windows_sys::Win32::{
    Foundation::{LPARAM, LRESULT, POINT, WPARAM},
    UI::{
        HiDpi::{SetProcessDpiAwarenessContext, DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2},
        WindowsAndMessaging::*,
    },
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ControlState {
    Local,
    Remote,
    Disconnected,
}

#[derive(Debug, Clone, Copy)]
enum HookEvent {
    Input(InputEvent),
    EmergencyStop,
}

static HOOK_EVENTS: OnceLock<Sender<HookEvent>> = OnceLock::new();
static REMOTE_INPUT: AtomicBool = AtomicBool::new(false);
static MOUSE_ANCHOR_X: AtomicI32 = AtomicI32::new(0);
static MOUSE_ANCHOR_Y: AtomicI32 = AtomicI32::new(0);
static CTRL_DOWN: AtomicBool = AtomicBool::new(false);
static ALT_DOWN: AtomicBool = AtomicBool::new(false);

fn queue_hook_event(event: HookEvent) {
    if let Some(tx) = HOOK_EVENTS.get() {
        let _ = tx.try_send(event);
    }
}

fn set_remote_input(enabled: bool) {
    if enabled {
        let mut point = POINT { x: 0, y: 0 };
        if unsafe { GetCursorPos(&mut point) } != 0 {
            MOUSE_ANCHOR_X.store(point.x, Ordering::Release);
            MOUSE_ANCHOR_Y.store(point.y, Ordering::Release);
        }
        CTRL_DOWN.store(false, Ordering::Release);
        ALT_DOWN.store(false, Ordering::Release);
    }
    REMOTE_INPUT.store(enabled, Ordering::Release);
}

fn windows_key_to_linux(vk: u32, scan_code: u32) -> u16 {
    const LETTERS: [u16; 26] = [
        30, 48, 46, 32, 18, 33, 34, 35, 23, 36, 37, 38, 50, 49, 24, 25, 16, 19, 31, 20, 22, 47, 17,
        45, 21, 44,
    ];
    const DIGITS: [u16; 10] = [11, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    match vk {
        0x41..=0x5A => LETTERS[(vk - 0x41) as usize],
        0x30..=0x39 => DIGITS[(vk - 0x30) as usize],
        0x1B => 1,
        0x08 => 14,
        0x09 => 15,
        0x0D => 28,
        0x20 => 57,
        0xA2 | 0xA3 => 29,
        0xA0 | 0xA1 => 42,
        0xA4 | 0xA5 => 56,
        0x5B | 0x5C => 125,
        0x25 => 105,
        0x26 => 103,
        0x27 => 106,
        0x28 => 108,
        _ => scan_code as u16,
    }
}

unsafe extern "system" fn mouse_hook(code: i32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    if code != HC_ACTION as i32 || !REMOTE_INPUT.load(Ordering::Acquire) {
        return CallNextHookEx(0, code, wparam, lparam);
    }

    let data = &*(lparam as *const MSLLHOOKSTRUCT);
    let message = wparam as u32;
    let event = match message {
        WM_MOUSEMOVE => {
            let dx = (data.pt.x - MOUSE_ANCHOR_X.load(Ordering::Acquire))
                .clamp(i16::MIN as i32, i16::MAX as i32) as i16;
            let dy = (data.pt.y - MOUSE_ANCHOR_Y.load(Ordering::Acquire))
                .clamp(i16::MIN as i32, i16::MAX as i32) as i16;
            (dx != 0 || dy != 0).then_some(InputEvent::MouseMove { dx, dy })
        }
        WM_LBUTTONDOWN => Some(InputEvent::MouseButton {
            button: 1,
            pressed: true,
        }),
        WM_LBUTTONUP => Some(InputEvent::MouseButton {
            button: 1,
            pressed: false,
        }),
        WM_RBUTTONDOWN => Some(InputEvent::MouseButton {
            button: 2,
            pressed: true,
        }),
        WM_RBUTTONUP => Some(InputEvent::MouseButton {
            button: 2,
            pressed: false,
        }),
        WM_MBUTTONDOWN => Some(InputEvent::MouseButton {
            button: 3,
            pressed: true,
        }),
        WM_MBUTTONUP => Some(InputEvent::MouseButton {
            button: 3,
            pressed: false,
        }),
        WM_XBUTTONDOWN | WM_XBUTTONUP => Some(InputEvent::MouseButton {
            button: if ((data.mouseData >> 16) & 0xffff) == 1 {
                4
            } else {
                5
            },
            pressed: message == WM_XBUTTONDOWN,
        }),
        WM_MOUSEWHEEL => Some(InputEvent::MouseWheel {
            vertical: ((data.mouseData >> 16) as i16) / 120,
            horizontal: 0,
        }),
        WM_MOUSEHWHEEL => Some(InputEvent::MouseWheel {
            vertical: 0,
            horizontal: ((data.mouseData >> 16) as i16) / 120,
        }),
        _ => None,
    };

    if let Some(event) = event {
        queue_hook_event(HookEvent::Input(event));
    }

    if matches!(
        message,
        WM_MOUSEMOVE
            | WM_LBUTTONDOWN
            | WM_LBUTTONUP
            | WM_RBUTTONDOWN
            | WM_RBUTTONUP
            | WM_MBUTTONDOWN
            | WM_MBUTTONUP
            | WM_XBUTTONDOWN
            | WM_XBUTTONUP
            | WM_MOUSEWHEEL
            | WM_MOUSEHWHEEL
    ) {
        1
    } else {
        CallNextHookEx(0, code, wparam, lparam)
    }
}

unsafe extern "system" fn keyboard_hook(code: i32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    if code != HC_ACTION as i32 || !REMOTE_INPUT.load(Ordering::Acquire) {
        return CallNextHookEx(0, code, wparam, lparam);
    }

    let data = &*(lparam as *const KBDLLHOOKSTRUCT);
    let message = wparam as u32;
    let pressed = matches!(message, WM_KEYDOWN | WM_SYSKEYDOWN);
    let released = matches!(message, WM_KEYUP | WM_SYSKEYUP);
    if !pressed && !released {
        return CallNextHookEx(0, code, wparam, lparam);
    }

    if matches!(data.vkCode, 0xA2 | 0xA3) {
        CTRL_DOWN.store(pressed, Ordering::Release);
    }
    if matches!(data.vkCode, 0xA4 | 0xA5) {
        ALT_DOWN.store(pressed, Ordering::Release);
    }
    if pressed
        && data.vkCode == 0x1B
        && CTRL_DOWN.load(Ordering::Acquire)
        && ALT_DOWN.load(Ordering::Acquire)
    {
        REMOTE_INPUT.store(false, Ordering::Release);
        queue_hook_event(HookEvent::EmergencyStop);
        return 1;
    }

    queue_hook_event(HookEvent::Input(InputEvent::Key {
        code: windows_key_to_linux(data.vkCode, data.scanCode),
        pressed,
    }));
    1
}

fn start_windows_hooks(tx: Sender<HookEvent>) {
    let _ = HOOK_EVENTS.set(tx);
    std::thread::spawn(|| unsafe {
        let mouse = SetWindowsHookExW(WH_MOUSE_LL, Some(mouse_hook), 0, 0);
        let keyboard = SetWindowsHookExW(WH_KEYBOARD_LL, Some(keyboard_hook), 0, 0);
        if mouse == 0 || keyboard == 0 {
            warn!("failed to install Windows input hooks");
            return;
        }
        let mut message = std::mem::zeroed::<MSG>();
        while GetMessageW(&mut message, 0, 0, 0) > 0 {
            TranslateMessage(&message);
            DispatchMessageW(&message);
        }
        UnhookWindowsHookEx(mouse);
        UnhookWindowsHookEx(keyboard);
    });
}

struct InputSender {
    socket: Arc<UdpSocket>,
    target: SocketAddr,
    session: u64,
    sequence: u32,
}

impl InputSender {
    async fn send(&mut self, event: InputEvent) -> Result<()> {
        self.sequence = self.sequence.wrapping_add(1);
        let packet = Packet::Input {
            session: self.session,
            sequence: self.sequence,
            event,
        };
        self.socket.send_to(&encode(&packet)?, self.target).await?;
        Ok(())
    }

    async fn state(&self, remote: bool) -> Result<()> {
        let packet = Packet::SetState {
            session: self.session,
            remote,
        };
        self.socket.send_to(&encode(&packet)?, self.target).await?;
        Ok(())
    }

    async fn enter_remote(
        &self,
        edge: ScreenEdge,
        ratio: f32,
        width: u32,
        height: u32,
    ) -> Result<()> {
        let packet = Packet::EnterRemote {
            session: self.session,
            edge,
            ratio: ratio.clamp(0.0, 1.0),
            width,
            height,
        };
        self.socket.send_to(&encode(&packet)?, self.target).await?;
        Ok(())
    }

    async fn hello(&self, token: &str) -> Result<()> {
        let packet = Packet::Hello {
            version: PROTOCOL_VERSION,
            device_id: "desklink-windows".to_owned(),
            token: token.to_owned(),
            session: self.session,
        };
        self.socket.send_to(&encode(&packet)?, self.target).await?;
        Ok(())
    }

    async fn release_all(&self) -> Result<()> {
        let packet = Packet::ReleaseAll {
            session: self.session,
        };
        self.socket.send_to(&encode(&packet)?, self.target).await?;
        Ok(())
    }
}

fn now_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u64
}

async fn flush_mouse(sender: &mut InputSender, dx: &mut i32, dy: &mut i32) -> Result<()> {
    if *dx == 0 && *dy == 0 {
        return Ok(());
    }
    let send_dx = (*dx).clamp(i16::MIN as i32, i16::MAX as i32) as i16;
    let send_dy = (*dy).clamp(i16::MIN as i32, i16::MAX as i32) as i16;
    *dx -= send_dx as i32;
    *dy -= send_dy as i32;
    sender
        .send(InputEvent::MouseMove {
            dx: send_dx,
            dy: send_dy,
        })
        .await
}

#[tokio::main]
async fn main() -> Result<()> {
    unsafe {
        SetProcessDpiAwarenessContext(DPI_AWARENESS_CONTEXT_PER_MONITOR_AWARE_V2);
    }
    tracing_subscriber::fmt().with_env_filter("info").init();
    let config_path =
        std::env::var("DESKLINK_CONFIG").unwrap_or_else(|_| "desklink.toml".to_owned());
    let mut config = DeskLinkConfig::load_optional(&config_path)?;
    let (initial_layout, windows_layout_changed) =
        LayoutSnapshot::from_config(&mut config.topology);
    config.save(&config_path)?;
    let target: SocketAddr = std::env::var("DESKLINK_TARGET")
        .unwrap_or_else(|_| config.network.target.clone())
        .parse()
        .context("invalid DESKLINK_TARGET")?;
    let token = std::env::var("DESKLINK_TOKEN").unwrap_or_else(|_| config.security.token.clone());
    let disconnect_timeout =
        Duration::from_millis(config.performance.disconnect_timeout_ms.max(1_000));
    let topology_enabled = config.topology.enabled;
    let enter_margin_px = config.topology.enter_margin_px;
    let edge_delay = Duration::from_millis(config.topology.edge_delay_ms);
    let return_cooldown = Duration::from_millis(config.topology.return_cooldown_ms);
    let ui_bind: SocketAddr = config
        .network
        .ui_bind
        .parse()
        .context("invalid network.ui_bind")?;
    let layout_state = Arc::new(RwLock::new(initial_layout));
    let config_state = Arc::new(RwLock::new(config.clone()));
    let ui_state = UiState {
        layout: layout_state.clone(),
        config: config_state.clone(),
        config_path: PathBuf::from(&config_path),
    };
    tokio::spawn(async move {
        if let Err(error) = web_ui::serve(ui_bind, ui_state).await {
            warn!(%error, "localhost layout UI stopped");
        }
    });
    if windows_layout_changed {
        warn!("Windows monitor layout changed; Linux screen placement was reset");
    }
    let socket = Arc::new(UdpSocket::bind("0.0.0.0:0").await?);
    let mut sender = InputSender {
        socket: socket.clone(),
        target,
        session: now_ms(),
        sequence: 0,
    };
    let mut state = ControlState::Local;
    let (hook_tx, mut hook_rx): (Sender<HookEvent>, Receiver<HookEvent>) = mpsc::channel(8192);
    start_windows_hooks(hook_tx);
    let (network_tx, mut network_rx) = mpsc::channel::<(Packet, SocketAddr)>(64);
    let receive_socket = socket.clone();
    tokio::spawn(async move {
        let mut buffer = [0u8; 2048];
        loop {
            let Ok((size, peer)) = receive_socket.recv_from(&mut buffer).await else {
                continue;
            };
            let Ok(packet) = decode(&buffer[..size]) else {
                continue;
            };
            if network_tx.send((packet, peer)).await.is_err() {
                break;
            }
        }
    });

    info!(?target, ?state, ui = %format!("http://{ui_bind}"), "DeskLink Windows Host started");
    let mut lines = io::BufReader::new(io::stdin()).lines();
    let mut stdin_open = true;
    let mut heartbeat = time::interval(Duration::from_secs(2));
    heartbeat.set_missed_tick_behavior(MissedTickBehavior::Skip);
    let mut move_flush = time::interval(Duration::from_millis(
        config.performance.mouse_flush_ms.clamp(1, 20),
    ));
    move_flush.set_missed_tick_behavior(MissedTickBehavior::Skip);
    let mut edge_poll = time::interval(Duration::from_millis(10));
    edge_poll.set_missed_tick_behavior(MissedTickBehavior::Skip);
    let mut pending_dx = 0i32;
    let mut pending_dy = 0i32;
    let mut last_ack: Option<Instant> = Some(Instant::now());
    let mut reported_connected = false;
    let mut connected = false;
    let mut edge_candidate: Option<(ScreenEdge, Instant)> = None;
    let mut last_edge_return = Instant::now()
        .checked_sub(return_cooldown)
        .unwrap_or_else(Instant::now);

    loop {
        tokio::select! {
            _ = heartbeat.tick() => {
                let current_signature = layout::layout_signature(&layout::discover_windows_screens());
                if current_signature != layout_state.read().await.signature {
                    set_remote_input(false);
                    state = ControlState::Local;
                    let mut live_config = config_state.write().await;
                    let (new_layout, _) = LayoutSnapshot::from_config(&mut live_config.topology);
                    live_config.save(&config_path)?;
                    *layout_state.write().await = new_layout;
                    edge_candidate = None;
                    warn!("Windows monitor layout changed; saved Linux placement was reset");
                }
                sender.hello(&token).await?;
                let packet = Packet::Heartbeat { session: sender.session, timestamp_ms: now_ms() };
                if let Err(error) = sender.socket.send_to(&encode(&packet)?, sender.target).await {
                    state = ControlState::Disconnected;
                    warn!(%error, ?state, "heartbeat failed");
                } else if state == ControlState::Remote {
                    sender.state(true).await?;
                }
                if last_ack.map(|seen| seen.elapsed() > disconnect_timeout).unwrap_or(true) {
                    if state == ControlState::Remote {
                        pending_dx = 0;
                        pending_dy = 0;
                        set_remote_input(false);
                        state = ControlState::Disconnected;
                        warn!(timeout_ms = disconnect_timeout.as_millis(), "Linux client ACK timed out; local input restored");
                    }
                    connected = false;
                    reported_connected = false;
                }
            }
            Some((packet, peer)) = network_rx.recv() => {
                if peer == sender.target {
                    match packet {
                        Packet::Ack { session } if session == sender.session => {
                            last_ack = Some(Instant::now());
                            connected = true;
                            if !reported_connected {
                                reported_connected = true;
                                info!("Linux client acknowledged; connection ready");
                            }
                        }
                        Packet::EdgeReturn { session, edge, ratio } if session == sender.session => {
                            pending_dx = 0;
                            pending_dy = 0;
                            set_remote_input(false);
                            sender.state(false).await?;
                            sender.release_all().await?;
                            if let Some((x, y)) = layout_state.read().await.return_point(edge, ratio) {
                                unsafe { SetCursorPos(x, y); }
                            }
                            state = ControlState::Local;
                            last_edge_return = Instant::now();
                            edge_candidate = None;
                            info!(?edge, ratio, "cursor returned to Windows");
                        }
                        Packet::Reject { session, reason } if session == sender.session => {
                            connected = false;
                            set_remote_input(false);
                            state = ControlState::Disconnected;
                            warn!(%reason, "Linux client rejected the connection");
                        }
                        _ => {}
                    }
                }
            }
            _ = edge_poll.tick(), if topology_enabled && state == ControlState::Local && connected => {
                if last_edge_return.elapsed() >= return_cooldown {
                    let mut point = POINT { x: 0, y: 0 };
                    let entry = if unsafe { GetCursorPos(&mut point) } != 0 {
                        layout_state.read().await.entry_at(point.x, point.y, enter_margin_px)
                    } else { None };
                    match entry {
                        Some((remote_edge, ratio)) => {
                            let since = match edge_candidate {
                                Some((edge, since)) if edge == remote_edge => since,
                                _ => {
                                    let now = Instant::now();
                                    edge_candidate = Some((remote_edge, now));
                                    now
                                }
                            };
                            if since.elapsed() >= edge_delay {
                                pending_dx = 0;
                                pending_dy = 0;
                                let remote = layout_state.read().await.remote.clone();
                                sender.enter_remote(remote_edge, ratio, remote.width, remote.height).await?;
                                sender.state(true).await?;
                                set_remote_input(true);
                                state = ControlState::Remote;
                                edge_candidate = None;
                                info!(?remote_edge, ratio, "cursor crossed into Linux");
                            }
                        }
                        None => edge_candidate = None,
                    }
                }
            }
            _ = move_flush.tick(), if state == ControlState::Remote => {
                flush_mouse(&mut sender, &mut pending_dx, &mut pending_dy).await?;
            }
            Some(hook_event) = hook_rx.recv() => {
                match hook_event {
                    HookEvent::Input(InputEvent::MouseMove { dx, dy }) if state == ControlState::Remote => {
                        pending_dx = pending_dx.saturating_add(dx as i32).clamp(-131_072, 131_072);
                        pending_dy = pending_dy.saturating_add(dy as i32).clamp(-131_072, 131_072);
                    }
                    HookEvent::Input(event) if state == ControlState::Remote => {
                        flush_mouse(&mut sender, &mut pending_dx, &mut pending_dy).await?;
                        sender.send(event).await?;
                    }
                    HookEvent::EmergencyStop => {
                        pending_dx = 0;
                        pending_dy = 0;
                        set_remote_input(false);
                        sender.state(false).await?;
                        sender.release_all().await?;
                        state = ControlState::Local;
                        warn!("emergency hotkey activated; local input restored");
                    }
                    _ => {}
                }
            }
            line = lines.next_line(), if stdin_open => {
                let Some(line) = line? else { stdin_open = false; continue };
                let mut parts = line.split_whitespace();
                match parts.next().unwrap_or("") {
                    "move" => {
                        let dx: i16 = parts.next().unwrap_or("0").parse()?;
                        let dy: i16 = parts.next().unwrap_or("0").parse()?;
                        sender.send(InputEvent::MouseMove { dx, dy }).await?;
                    }
                    "click" => {
                        let button: u8 = parts.next().unwrap_or("1").parse()?;
                        sender.send(InputEvent::MouseButton { button, pressed: true }).await?;
                        sender.send(InputEvent::MouseButton { button, pressed: false }).await?;
                    }
                    "key" => {
                        let code: u16 = parts.next().unwrap_or("30").parse()?;
                        let pressed = parts.next().unwrap_or("down") != "up";
                        sender.send(InputEvent::Key { code, pressed }).await?;
                    }
                    "remote" => {
                        pending_dx = 0;
                        pending_dy = 0;
                        last_ack = Some(Instant::now());
                        reported_connected = false;
                        if let Some((edge, ratio)) = layout_state.read().await.default_entry() {
                            let remote = layout_state.read().await.remote.clone();
                            sender.enter_remote(edge, ratio, remote.width, remote.height).await?;
                        }
                        sender.state(true).await?;
                        set_remote_input(true);
                        state = ControlState::Remote;
                        info!(?state, "remote input enabled");
                    }
                    "local" | "release" => {
                        pending_dx = 0;
                        pending_dy = 0;
                        set_remote_input(false);
                        sender.state(false).await?;
                        sender.release_all().await?;
                        state = ControlState::Local;
                        info!(?state, "local input restored");
                    }
                    "help" => println!("move DX DY | click BUTTON | key CODE down/up | remote | local | release | quit"),
                    "quit" => break,
                    _ => println!("unknown command; type help"),
                }
            }
        }
    }

    set_remote_input(false);
    Ok(())
}

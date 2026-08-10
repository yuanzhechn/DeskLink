use anyhow::{Context, Result};
use desklink_config::DeskLinkConfig;
use desklink_protocol::{decode, encode, InputEvent, Packet, ScreenEdge, PROTOCOL_VERSION};
use evdev::{
    uinput::VirtualDeviceBuilder, AttributeSet, EventType, InputEvent as EvdevEvent, Key,
    RelativeAxisType,
};
use std::{collections::HashSet, net::SocketAddr, time::Instant};
use tokio::net::UdpSocket;
use tokio::time::{timeout, Duration};
use tracing::{info, warn};

struct Injector {
    keyboard: evdev::uinput::VirtualDevice,
    mouse: evdev::uinput::VirtualDevice,
    pressed: HashSet<u16>,
    pressed_buttons: HashSet<u16>,
}

struct RemoteCursor {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    return_edge: Option<ScreenEdge>,
}

impl RemoteCursor {
    fn new(width: u32, height: u32) -> Self {
        Self {
            x: width as i32 / 2,
            y: height as i32 / 2,
            width: width.max(1) as i32,
            height: height.max(1) as i32,
            return_edge: None,
        }
    }

    fn enter(&mut self, edge: ScreenEdge, ratio: f32, width: u32, height: u32) {
        self.width = width.max(1) as i32;
        self.height = height.max(1) as i32;
        let ratio = ratio.clamp(0.0, 1.0);
        match edge {
            ScreenEdge::Left => {
                self.x = 1;
                self.y = (self.height as f32 * ratio) as i32;
            }
            ScreenEdge::Right => {
                self.x = self.width - 2;
                self.y = (self.height as f32 * ratio) as i32;
            }
            ScreenEdge::Top => {
                self.x = (self.width as f32 * ratio) as i32;
                self.y = 1;
            }
            ScreenEdge::Bottom => {
                self.x = (self.width as f32 * ratio) as i32;
                self.y = self.height - 2;
            }
        }
        self.x = self.x.clamp(0, self.width - 1);
        self.y = self.y.clamp(0, self.height - 1);
        self.return_edge = Some(edge);
    }

    fn move_by(&mut self, dx: i16, dy: i16) -> Option<(ScreenEdge, f32)> {
        let next_x = self.x.saturating_add(dx as i32);
        let next_y = self.y.saturating_add(dy as i32);
        if let Some(edge) = self.return_edge {
            let crossed = match edge {
                ScreenEdge::Left => dx < 0 && next_x <= 0,
                ScreenEdge::Right => dx > 0 && next_x >= self.width - 1,
                ScreenEdge::Top => dy < 0 && next_y <= 0,
                ScreenEdge::Bottom => dy > 0 && next_y >= self.height - 1,
            };
            if crossed {
                let ratio = match edge {
                    ScreenEdge::Left | ScreenEdge::Right => self.y as f32 / self.height as f32,
                    ScreenEdge::Top | ScreenEdge::Bottom => self.x as f32 / self.width as f32,
                };
                return Some((edge, ratio.clamp(0.0, 1.0)));
            }
        }
        self.x = next_x.clamp(0, self.width - 1);
        self.y = next_y.clamp(0, self.height - 1);
        None
    }
}
impl Injector {
    fn new() -> Result<Self> {
        let mut keyboard_keys = AttributeSet::<Key>::new();
        for code in 1..=255 {
            keyboard_keys.insert(Key::new(code));
        }
        let keyboard = VirtualDeviceBuilder::new()?
            .name("DeskLink Virtual Keyboard")
            .with_keys(&keyboard_keys)?
            .build()?;

        let mut mouse_keys = AttributeSet::<Key>::new();
        mouse_keys.insert(Key::BTN_LEFT);
        mouse_keys.insert(Key::BTN_RIGHT);
        mouse_keys.insert(Key::BTN_MIDDLE);
        mouse_keys.insert(Key::BTN_SIDE);
        mouse_keys.insert(Key::BTN_EXTRA);
        let mut axes = AttributeSet::<RelativeAxisType>::new();
        axes.insert(RelativeAxisType::REL_X);
        axes.insert(RelativeAxisType::REL_Y);
        axes.insert(RelativeAxisType::REL_WHEEL);
        axes.insert(RelativeAxisType::REL_HWHEEL);
        let mouse = VirtualDeviceBuilder::new()?
            .name("DeskLink Virtual Mouse")
            .with_keys(&mouse_keys)?
            .with_relative_axes(&axes)?
            .build()?;
        Ok(Self {
            keyboard,
            mouse,
            pressed: HashSet::new(),
            pressed_buttons: HashSet::new(),
        })
    }
    fn emit(&mut self, event: InputEvent) -> Result<()> {
        let mut out = Vec::new();
        let keyboard_event;
        match event {
            InputEvent::MouseMove { dx, dy } => {
                keyboard_event = false;
                out.push(EvdevEvent::new(
                    EventType::RELATIVE,
                    RelativeAxisType::REL_X.0,
                    dx as i32,
                ));
                out.push(EvdevEvent::new(
                    EventType::RELATIVE,
                    RelativeAxisType::REL_Y.0,
                    dy as i32,
                ));
            }
            InputEvent::MouseWheel {
                vertical,
                horizontal,
            } => {
                keyboard_event = false;
                if vertical != 0 {
                    out.push(EvdevEvent::new(
                        EventType::RELATIVE,
                        RelativeAxisType::REL_WHEEL.0,
                        vertical as i32,
                    ));
                }
                if horizontal != 0 {
                    out.push(EvdevEvent::new(
                        EventType::RELATIVE,
                        RelativeAxisType::REL_HWHEEL.0,
                        horizontal as i32,
                    ));
                }
            }
            InputEvent::MouseButton { button, pressed } => {
                keyboard_event = false;
                let code = match button {
                    1 => Key::BTN_LEFT,
                    2 => Key::BTN_RIGHT,
                    3 => Key::BTN_MIDDLE,
                    4 => Key::BTN_SIDE,
                    _ => Key::BTN_EXTRA,
                };
                if pressed {
                    self.pressed_buttons.insert(code.code());
                } else {
                    self.pressed_buttons.remove(&code.code());
                }
                out.push(EvdevEvent::new(EventType::KEY, code.code(), pressed as i32));
            }
            InputEvent::Key { code, pressed } => {
                keyboard_event = true;
                if pressed {
                    self.pressed.insert(code);
                } else {
                    self.pressed.remove(&code);
                }
                out.push(EvdevEvent::new(EventType::KEY, code, pressed as i32));
            }
        }
        if !out.is_empty() {
            if keyboard_event {
                self.keyboard.emit(&out)?;
            } else {
                self.mouse.emit(&out)?;
            }
        }
        Ok(())
    }
    fn release_all(&mut self) -> Result<()> {
        for code in self.pressed.drain() {
            self.keyboard
                .emit(&[EvdevEvent::new(EventType::KEY, code, 0)])?;
        }
        for code in self.pressed_buttons.drain() {
            self.mouse
                .emit(&[EvdevEvent::new(EventType::KEY, code, 0)])?;
        }
        Ok(())
    }
}

fn sequence_is_newer(sequence: u32, previous: u32) -> bool {
    sequence != previous && sequence.wrapping_sub(previous) < (u32::MAX / 2)
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt().with_env_filter("info").init();
    let config_path =
        std::env::var("DESKLINK_CONFIG").unwrap_or_else(|_| "desklink.toml".to_owned());
    let config = DeskLinkConfig::load_optional(&config_path)?;
    let bind: SocketAddr = std::env::var("DESKLINK_BIND")
        .unwrap_or_else(|_| config.network.bind.clone())
        .parse()?;
    let token = std::env::var("DESKLINK_TOKEN").unwrap_or_else(|_| config.security.token.clone());
    let disconnect_timeout =
        Duration::from_millis(config.performance.disconnect_timeout_ms.max(1_000));
    let socket = UdpSocket::bind(bind).await.context("bind input port")?;
    let mut injector =
        Injector::new().context("create /dev/uinput device (check udev permissions)")?;
    let mut cursor = RemoteCursor::new(config.topology.remote_width, config.topology.remote_height);
    let mut buf = [0u8; 2048];
    let mut remote = false;
    let mut authorized: Option<(SocketAddr, u64)> = None;
    let mut sequence: Option<u32> = None;
    let mut last_seen = Instant::now();
    info!(?bind, "DeskLink Linux Client listening");
    loop {
        let (n, peer) = match timeout(Duration::from_secs(1), socket.recv_from(&mut buf)).await {
            Ok(result) => result?,
            Err(_) => {
                if authorized.is_some() && last_seen.elapsed() > disconnect_timeout {
                    if remote {
                        injector.release_all()?;
                    }
                    remote = false;
                    authorized = None;
                    sequence = None;
                    warn!("host timeout; session cleared and local state restored");
                }
                continue;
            }
        };
        let packet = match decode(&buf[..n]) {
            Ok(p) => p,
            Err(e) => {
                warn!(%e, "invalid packet");
                continue;
            }
        };
        match packet {
            Packet::Hello {
                version, session, ..
            } if version != PROTOCOL_VERSION => {
                socket
                    .send_to(
                        &encode(&Packet::Reject {
                            session,
                            reason: format!(
                                "protocol mismatch: host={version}, linux={PROTOCOL_VERSION}"
                            ),
                        })?,
                        peer,
                    )
                    .await?;
                warn!(?peer, version, "rejected incompatible Windows host");
            }
            Packet::Hello {
                token: supplied,
                session,
                ..
            } if supplied != token => {
                socket
                    .send_to(
                        &encode(&Packet::Reject {
                            session,
                            reason: "shared token mismatch".to_owned(),
                        })?,
                        peer,
                    )
                    .await?;
                warn!(?peer, "rejected Windows host with wrong token");
            }
            Packet::Hello {
                version,
                token: supplied,
                session,
                ..
            } if version == PROTOCOL_VERSION && supplied == token => {
                if authorized != Some((peer, session)) {
                    if remote {
                        injector.release_all()?;
                    }
                    remote = false;
                    sequence = None;
                    authorized = Some((peer, session));
                    info!(?peer, session, "Windows host authorized");
                }
                last_seen = Instant::now();
                socket
                    .send_to(&encode(&Packet::Ack { session })?, peer)
                    .await?;
            }
            Packet::Heartbeat { session, .. } if authorized == Some((peer, session)) => {
                last_seen = Instant::now();
                socket
                    .send_to(&encode(&Packet::Ack { session })?, peer)
                    .await?;
            }
            Packet::Input {
                session,
                sequence: current,
                event,
            } if remote && authorized == Some((peer, session)) => {
                last_seen = Instant::now();
                if sequence
                    .map(|previous| sequence_is_newer(current, previous))
                    .unwrap_or(true)
                {
                    sequence = Some(current);
                    if let InputEvent::MouseMove { dx, dy } = event {
                        if let Some((edge, ratio)) = cursor.move_by(dx, dy) {
                            remote = false;
                            injector.release_all()?;
                            socket
                                .send_to(
                                    &encode(&Packet::EdgeReturn {
                                        session,
                                        edge,
                                        ratio,
                                    })?,
                                    peer,
                                )
                                .await?;
                            info!(?edge, ratio, "cursor returned to Windows edge");
                            continue;
                        }
                    }
                    injector.emit(event)?;
                }
            }
            Packet::EnterRemote {
                session,
                edge,
                ratio,
                width,
                height,
            } if authorized == Some((peer, session)) => {
                cursor.enter(edge, ratio, width, height);
                remote = true;
                sequence = None;
                last_seen = Instant::now();
                socket
                    .send_to(&encode(&Packet::Ack { session })?, peer)
                    .await?;
                info!(?edge, ratio, "cursor entered Linux screen");
            }
            Packet::SetState {
                session,
                remote: requested,
            } if authorized == Some((peer, session)) => {
                last_seen = Instant::now();
                if remote != requested {
                    if !requested {
                        injector.release_all()?;
                    }
                    remote = requested;
                    info!(?remote, "input state changed");
                }
                socket
                    .send_to(&encode(&Packet::Ack { session })?, peer)
                    .await?;
            }
            Packet::ReleaseAll { session } if authorized == Some((peer, session)) => {
                last_seen = Instant::now();
                injector.release_all()?;
            }
            _ => {}
        }
        if authorized.is_some() && last_seen.elapsed() > disconnect_timeout {
            if remote {
                injector.release_all()?;
            }
            remote = false;
            authorized = None;
            sequence = None;
            warn!("host timeout; session cleared and local state restored");
        }
    }
}

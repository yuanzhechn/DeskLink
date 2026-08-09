use anyhow::{Context, Result};
use desklink_protocol::{decode, InputEvent, Packet, DEFAULT_INPUT_PORT};
use evdev::{uinput::VirtualDeviceBuilder, AttributeSet, EventType, InputEvent as EvdevEvent, Key, RelativeAxisType};
use std::{collections::HashSet, net::SocketAddr};
use tokio::net::UdpSocket;
use tokio::time::{timeout, Duration};
use tracing::{info, warn};

struct Injector { device: evdev::uinput::VirtualDevice, pressed: HashSet<u16> }
impl Injector {
    fn new() -> Result<Self> {
        let mut keys = AttributeSet::<Key>::new();
        for code in 1..=255 { keys.insert(Key::new(code)); }
        let mut axes = AttributeSet::<RelativeAxisType>::new();
        axes.insert(RelativeAxisType::REL_X);
        axes.insert(RelativeAxisType::REL_Y);
        axes.insert(RelativeAxisType::REL_WHEEL);
        axes.insert(RelativeAxisType::REL_HWHEEL);
        let device = VirtualDeviceBuilder::new()?.name("DeskLink Virtual Input").with_keys(&keys)?.with_relative_axes(&axes)?.build()?;
        Ok(Self { device, pressed: HashSet::new() })
    }
    fn emit(&mut self, event: InputEvent) -> Result<()> {
        let mut out = Vec::new();
        match event {
            InputEvent::MouseMove { dx, dy } => { out.push(EvdevEvent::new(EventType::RELATIVE, RelativeAxisType::REL_X.0, dx as i32)); out.push(EvdevEvent::new(EventType::RELATIVE, RelativeAxisType::REL_Y.0, dy as i32)); }
            InputEvent::MouseWheel { vertical, horizontal } => { if vertical != 0 { out.push(EvdevEvent::new(EventType::RELATIVE, RelativeAxisType::REL_WHEEL.0, vertical as i32)); } if horizontal != 0 { out.push(EvdevEvent::new(EventType::RELATIVE, RelativeAxisType::REL_HWHEEL.0, horizontal as i32)); } }
            InputEvent::MouseButton { button, pressed } => { let code = match button { 1 => Key::BTN_LEFT, 2 => Key::BTN_RIGHT, 3 => Key::BTN_MIDDLE, 4 => Key::BTN_SIDE, _ => Key::BTN_EXTRA }; out.push(EvdevEvent::new(EventType::KEY, code.code(), pressed as i32)); }
            InputEvent::Key { code, pressed } => { if pressed { self.pressed.insert(code); } else { self.pressed.remove(&code); } out.push(EvdevEvent::new(EventType::KEY, code, pressed as i32)); }
        }
        if !out.is_empty() { self.device.emit(&out)?; }
        Ok(())
    }
    fn release_all(&mut self) -> Result<()> { for code in self.pressed.drain() { self.device.emit(&[EvdevEvent::new(EventType::KEY, code, 0)])?; } Ok(()) }
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt().with_env_filter("info").init();
    let bind: SocketAddr = std::env::var("DESKLINK_BIND").unwrap_or_else(|_| format!("0.0.0.0:{DEFAULT_INPUT_PORT}")).parse()?;
    let socket = UdpSocket::bind(bind).await.context("bind input port")?;
    let mut injector = Injector::new().context("create /dev/uinput device (check udev permissions)")?;
    let mut buf = [0u8; 2048]; let mut remote = false; let mut session = 0; let mut sequence = 0;
    info!(?bind, "DeskLink Linux Client listening");
    loop {
        let (n, _peer) = match timeout(Duration::from_secs(5), socket.recv_from(&mut buf)).await {
            Ok(result) => result?,
            Err(_) => {
                if remote { remote = false; injector.release_all()?; warn!("host timeout; returned to local state"); }
                continue;
            }
        };
        let packet = match decode(&buf[..n]) { Ok(p) => p, Err(e) => { warn!(%e, "invalid packet"); continue; } };
        match packet {
            Packet::Input { session: s, sequence: seq, event } if remote || session == 0 => { if s != session { session = s; sequence = 0; } if seq > sequence { sequence = seq; injector.emit(event)?; } }
            Packet::SetState { session: s, remote: r } => { session = s; remote = r; if !remote { injector.release_all()?; } info!(?remote, "input state changed"); }
            Packet::ReleaseAll { .. } => injector.release_all()?,
            _ => {}
        }
    }
}

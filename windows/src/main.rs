use anyhow::{Context, Result};
use desklink_protocol::{encode, InputEvent, Packet, DEFAULT_INPUT_PORT};
use std::{net::SocketAddr, time::{Duration, SystemTime, UNIX_EPOCH}};
use tokio::{io::{self, AsyncBufReadExt}, net::UdpSocket, time};
use tracing::{info, warn};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ControlState { Local, SwitchingToRemote, Remote, SwitchingToLocal, Disconnected }

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
    info!(?target, "DeskLink Windows Host started (type 'help' for commands)");
    let mut lines = io::BufReader::new(io::stdin()).lines();
    let mut heartbeat = time::interval(Duration::from_secs(2));
    loop {
        tokio::select! {
            _ = heartbeat.tick() => { let p = Packet::Heartbeat { session: sender.session, timestamp_ms: now_ms() }; if let Err(e) = sender.socket.send_to(&encode(&p)?, sender.target).await { warn!(%e, "heartbeat failed"); state = ControlState::Disconnected; } }
            line = lines.next_line() => {
                let Some(line) = line? else { break };
                let mut p = line.split_whitespace();
                match p.next().unwrap_or("") {
                    "move" => { let dx: i16 = p.next().unwrap_or("0").parse()?; let dy: i16 = p.next().unwrap_or("0").parse()?; sender.send(InputEvent::MouseMove { dx, dy }).await?; }
                    "click" => { let button: u8 = p.next().unwrap_or("1").parse()?; sender.send(InputEvent::MouseButton { button, pressed: true }).await?; sender.send(InputEvent::MouseButton { button, pressed: false }).await?; }
                    "key" => { let code: u16 = p.next().unwrap_or("30").parse()?; let pressed = p.next().unwrap_or("down") != "up"; sender.send(InputEvent::Key { code, pressed }).await?; }
                    "remote" => { state = ControlState::SwitchingToRemote; sender.state(true).await?; state = ControlState::Remote; info!(?state, "remote input enabled"); }
                    "local" | "release" => { sender.state(false).await?; let p = Packet::ReleaseAll { session: sender.session }; sender.socket.send_to(&encode(&p)?, sender.target).await?; state = ControlState::Local; info!(?state, "local input restored"); }
                    "help" => println!("move DX DY | click BUTTON | key CODE down/up | remote | local | release | quit"),
                    "quit" => break,
                    _ => println!("unknown command; type help"),
                }
            }
        }
    }
    Ok(())
}


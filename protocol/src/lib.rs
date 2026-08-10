use serde::{Deserialize, Serialize};

pub const PROTOCOL_VERSION: u8 = 2;
pub const DEFAULT_INPUT_PORT: u16 = 24801;
pub const DEFAULT_CONTROL_PORT: u16 = 24800;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum InputEvent {
    MouseMove { dx: i16, dy: i16 },
    MouseButton { button: u8, pressed: bool },
    MouseWheel { vertical: i16, horizontal: i16 },
    Key { code: u16, pressed: bool },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScreenEdge {
    Left,
    Right,
    Top,
    Bottom,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Packet {
    Hello {
        version: u8,
        device_id: String,
        token: String,
        session: u64,
    },
    Input {
        session: u64,
        sequence: u32,
        event: InputEvent,
    },
    Heartbeat {
        session: u64,
        timestamp_ms: u64,
    },
    ReleaseAll {
        session: u64,
    },
    SetState {
        session: u64,
        remote: bool,
    },
    EnterRemote {
        session: u64,
        edge: ScreenEdge,
        ratio: f32,
        width: u32,
        height: u32,
    },
    EdgeReturn {
        session: u64,
        edge: ScreenEdge,
        ratio: f32,
    },
    Ack {
        session: u64,
    },
    Reject {
        session: u64,
        reason: String,
    },
}

pub fn encode(packet: &Packet) -> Result<Vec<u8>, bincode::Error> {
    bincode::serialize(packet)
}
pub fn decode(bytes: &[u8]) -> Result<Packet, bincode::Error> {
    bincode::deserialize(bytes)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn input_packet_round_trips() {
        let packet = Packet::Input {
            session: 42,
            sequence: 7,
            event: InputEvent::MouseMove { dx: -12, dy: 8 },
        };
        assert!(matches!(
            decode(&encode(&packet).unwrap()).unwrap(),
            Packet::Input {
                session: 42,
                sequence: 7,
                ..
            }
        ));
    }

    #[test]
    fn hello_packet_round_trips() {
        let packet = Packet::Hello {
            version: PROTOCOL_VERSION,
            device_id: "windows-main".into(),
            token: "test-token".into(),
            session: 99,
        };
        assert!(matches!(
            decode(&encode(&packet).unwrap()).unwrap(),
            Packet::Hello { session: 99, .. }
        ));
    }
}

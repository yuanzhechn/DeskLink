# DeskLink Changelog

## 0.4.0

- Split Linux uinput into dedicated virtual keyboard and virtual mouse devices.
- Declare mouse button capabilities correctly.
- Release held keyboard keys and mouse buttons on disconnect.
- Add TOML configuration with environment-variable overrides.

## 0.3.0

- Add protocol handshake, shared token, peer/session authorization and ACK replies.
- Detect a missing Linux client from Windows instead of relying on UDP send success.
- Handle wrapped UDP sequence numbers.
- Allow either endpoint to start first and automatically reconnect.

## 0.2.0

- Fix Windows low-level mouse-hook jitter by using a fixed cursor anchor.
- Always suppress handled local mouse events in remote mode.
- Add a bounded hook queue and 2 ms mouse-motion coalescing.
- Add immediate `Ctrl+Alt+Esc` emergency recovery.

## 0.1.0

- Initial Windows-to-Linux UDP input prototype.


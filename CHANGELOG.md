# DeskLink Changelog

## 0.5.0

- Model every Windows monitor as an independent rectangle with signed coordinates.
- Model Linux as a draggable logical screen and generate EdgeLink records only for actual overlapping borders.
- Switch automatically from Windows to Linux after the configured edge delay.
- Maintain Linux remote cursor state without querying Wayland and return through the matching edge.
- Add a localhost-only layout editor at `http://127.0.0.1:24802`.
- Snap the Linux screen to nearby Windows edges and persist placement in `desklink.toml`.
- Detect Windows monitor-layout changes and invalidate stale placement automatically.

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

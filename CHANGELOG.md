# DeskLink Changelog

## 0.6.1

- Keep the clipboard TCP channel connected when an operating-system clipboard operation fails.
- Detect Wayland and X11 sessions and support both `wl-clipboard` and `xclip` backends.
- Include backend error details in Linux logs and retry transient Windows clipboard locks.

## 0.6.0

- Add bidirectional UTF-8 text clipboard synchronization over a separate reliable TCP channel.
- Authenticate the clipboard channel with the existing shared token and protocol version.
- Use native Windows clipboard APIs and `wl-clipboard` on Wayland Linux.
- Reconnect automatically regardless of endpoint startup order and suppress clipboard echo loops.
- Make the clipboard polling interval and transfer size configurable, with a 64 MiB default.

## 0.5.4

- Drive the visible Linux pointer from the maintained logical cursor using absolute coordinates for every move.
- Eliminate drift caused by Linux pointer acceleration on relative uinput events.
- Snap the visible Linux pointer to the exact return edge before sending `EdgeReturn`.

## 0.5.3

- Move absolute positioning to a dedicated uinput pointer device with `INPUT_PROP_POINTER`.
- Preserve the exact global crossing coordinate when mapping through partial or offset EdgeLinks.
- Add topology tests for aligned midpoint and vertically offset screen mapping.

## 0.5.2

- Log protocol version, effective config path and a non-secret token fingerprint on both endpoints.
- Align the visible Linux pointer with the logical entry edge through uinput absolute axes.
- Arm edge return only after moving at least 48 pixels into the Linux screen, preventing premature return.

## 0.5.1

- Bump the wire protocol version after the EdgeLink packet changes.
- Return explicit protocol-version and shared-token rejection reasons.
- Clean obsolete manual input-debug instructions from README.

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

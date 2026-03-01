# CAN Bus Examples

Source: https://github.com/socketcan-rs/socketcan-rs
License: MIT/Apache-2.0

## Files

- `blocking.rs` — Simple blocking CAN read/write, best starting point
- `echo.rs` — Echo received frames back, good for testing
- `echo_fd.rs` — Echo with CAN FD (flexible data rate)
- `enumerate.rs` — List available CAN interfaces
- `nonblocking.rs` — Non-blocking CAN I/O
- `fd_send.rs` — Send CAN FD frames
- `playlog.rs` — Replay a CAN log file
- `tokio_send.rs` — Async CAN send with tokio
- `tokio_bridge.rs` — Bridge two CAN interfaces with tokio
- `tokio_average.rs` — Tokio example computing averages from CAN data
- `tokio_print_frames.rs` — Print all CAN frames async
- `smol_send.rs` — Async CAN send with smol
- `smol_bridge.rs` — Bridge two CAN interfaces with smol
- `smol_print_frames.rs` — Print all CAN frames with smol
- `async_std_send.rs` — Async CAN send with async-std
- `async_std_bridge.rs` — Bridge with async-std
- `async_std_print_frames.rs` — Print frames with async-std

## Hardware Target
Raspberry Pi 5 or Jetson Orin with USB-CAN adapter or MCP2515 SPI CAN module.
Use for ODrive, VESC, or any CAN-based BLDC FOC controller.
Start with `blocking.rs` then move to `tokio_send.rs`.

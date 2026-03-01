# Modbus Examples

Source: https://github.com/slowtec/tokio-modbus
License: MIT/Apache-2.0

## Files

- `rtu-client.rs` — Modbus RTU client (use this for Leadshine drives over RS485)
- `rtu-client-sync.rs` — Synchronous version of RTU client
- `rtu-server.rs` — Modbus RTU server
- `rtu-server-address.rs` — RTU server with custom address
- `rtu-over-tcp-server.rs` — RTU tunneled over TCP
- `tcp-client.rs` — Modbus TCP client
- `tcp-client-sync.rs` — Synchronous TCP client
- `tcp-client-custom-fn.rs` — TCP client with custom function codes
- `tcp-server.rs` — Modbus TCP server
- `tls-client.rs` — Modbus TLS client
- `tls-server.rs` — Modbus TLS server

## Hardware Target
Raspberry Pi 5 (or any Linux node in the rack) with RS485 USB adapter or direct UART.

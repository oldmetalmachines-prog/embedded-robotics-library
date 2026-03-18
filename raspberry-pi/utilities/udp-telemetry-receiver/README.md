# UDP Telemetry Receiver - Raspberry Pi

## Purpose

Receives UDP packets from ESP32 or other embedded senders and logs them to JSONL, CSV, or both.

This is the companion to `esp32/communication/wifi/udp-telemetry-sender/`, but it is intentionally generic: it does not decode a telemetry schema yet. It records packet metadata plus the payload as UTF-8 when possible, or as a hex string when the payload is binary.

Use cases:
- Capture telemetry traffic during bring-up
- Log raw packets for later parsing and replay
- Verify network connectivity between sender and receiver
- Produce JSONL and CSV artifacts for debugging

---

## Hardware Required

- Raspberry Pi 5, Raspberry Pi Zero 2W, or any Linux machine
- Network connection shared with the sender

No external wiring is required.

---

## Software Dependencies

This crate currently uses only the Rust standard library.

```toml
[dependencies]
```

Platform: any Linux system with Rust installed.

---

## Build and Run

```bash
# Build release version
cargo build --release

# Run with defaults
cargo run --release -- --bind 0.0.0.0:9000 --mode both

# Write JSONL only
cargo run --release -- --bind 0.0.0.0:9000 --mode jsonl --jsonl telemetry.jsonl

# Write CSV only
cargo run --release -- --bind 0.0.0.0:9000 --mode csv --csv telemetry.csv

# One-packet smoke test
cargo run -- --bind 127.0.0.1:9101 --mode both --jsonl /tmp/udp-test.jsonl --csv /tmp/udp-test.csv --max-packets 1
```

---

## CLI Options

```text
--bind ADDR           UDP bind address (default: 0.0.0.0:9000)
--jsonl PATH          JSONL output path (default: telemetry.jsonl)
--csv PATH            CSV output path (default: telemetry.csv)
--mode MODE           jsonl | csv | both (default: both)
--buffer-size BYTES   receive buffer size in bytes (default: 2048)
--max-packets COUNT   stop after COUNT packets for test runs
-h, --help            show help
```

---

## Expected Output

### Startup

```text
udp-telemetry-receiver listening on 0.0.0.0:9000
mode=Both, jsonl=telemetry.jsonl, csv=telemetry.csv, buffer_size=2048, max_packets=None
```

### Test run with `--max-packets 1`

```text
udp-telemetry-receiver listening on 127.0.0.1:9101
mode=Both, jsonl=/tmp/udp-test.jsonl, csv=/tmp/udp-test.csv, buffer_size=2048, max_packets=Some(1)
received 1 packets, exiting due to --max-packets
```

### Example JSONL output

```json
{"ts_ms":1773806097651,"src":"127.0.0.1:50323","len":17,"payload_utf8":"{\"hello\":\"robot\"}"}
```

### Example CSV output

```csv
ts_ms,src_ip,src_port,len,payload_utf8
1773806097651,127.0.0.1,50323,17,"{""hello"":""robot""}"
```

---

## Troubleshooting

### No packets received

Check:

```bash
# Confirm the listener is bound to the expected port
ss -lun | grep 9000

# Send a quick local packet for validation
python3 -c "import socket; s=socket.socket(socket.AF_INET, socket.SOCK_DGRAM); s.sendto(b'{\"hello\":\"robot\"}', ('127.0.0.1', 9000))"
```

If the sender is an ESP32, verify its target IP and port match the receiver.

### Address already in use

```bash
sudo lsof -i :9000
```

Then either stop the other process or choose a different `--bind` port.

### Binary payloads show up as hex

That is expected behavior. Non-UTF-8 payloads are logged as a `0x...` string so bytes are preserved in a human-readable form.

---

## Features

- Configurable bind address, output paths, and buffer size
- Validated `jsonl | csv | both` output modes
- Persistent JSONL/CSV writers instead of reopening files per packet
- `--max-packets` for smoke tests and automation
- UTF-8 payload logging with binary fallback to hex
- Unit-tested argument parsing and escaping helpers

---

## Next Steps

Possible future improvements:
- Add optional telemetry-schema decoding for known packet formats
- Add rolling output files or size-based rotation
- Add packet counters and basic runtime stats
- Add replay tooling for captured JSONL/CSV logs

---

## Reference Documents

- Companion sender: `esp32/communication/wifi/udp-telemetry-sender/README.md`
- Protocol references: `common/protocols/`

# RTSP Camera Streamer (Jetson Orin Nano) — Rust + GStreamer

Streams a camera feed as **RTSP** using Rust bindings for GStreamer.

## What this is for
- Jetson Orin Nano Super dev kit (or any Linux host with GStreamer + RTSP server)
- Turn a CSI/USB camera into an RTSP endpoint on your LAN (view with VLC, ffplay, etc.)

## Requirements (Jetson / Ubuntu)
Install runtime + dev packages:

```bash

# J1939 Framework (C++)

Source: https://github.com/famez/J1939-Framework
Author: famez
License: GPL-3.0
Language: C++

## Purpose
Full J1939 framework for Linux with SocketCAN. Sniff, decode, send and
analyze J1939 frames from trucks, buses, farm equipment and industrial vehicles.

## Key Folders
- `BinUtils/` — Command line tools: sniffer, decoder, sender, address mapper
- `J1939/` — Core J1939 library (libJ1939.so)
- `CAN/` — CAN interface library supporting SocketCAN and PeakCAN
- `Scripts/` — Bash scripts for common J1939 tasks e.g. gear level simulation
- `Database/` — J1939 PGN/SPN database

## Tools
- `j1939Sniffer` — Sniff J1939 frames from CAN bus
- `j1939Decoder` — Decode raw J1939 data to human readable
- `j1939Sender` — Craft and send J1939 frames
- `j1939AddressMapper` — Discover J1939 devices on the bus

## Hardware Target
Raspberry Pi 5 or any Linux node with USB-CAN adapter.
Useful for diagnosing farm equipment and truck ECUs.

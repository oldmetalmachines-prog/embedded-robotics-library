# PyOBD - OBD-II Diagnostics (Python)

Source: https://github.com/barracuda-fsh/pyobd
Author: barracuda-fsh
License: GPL-2.0
Language: Python

## Purpose
Full OBD-II diagnostic tool for reading live sensor data, fault codes (DTCs),
and vehicle info via ELM327 adapter. Works on any car/light truck since 1996.

## Key Files
- `pyobd.py` — Main diagnostic application
- `obd_sensors.py` — All OBD-II sensor definitions and decoders
- `obd2_codes.py` — Full DTC fault code database
- `obd_io.py` — ELM327 serial communication
- `obd/` — Core OBD library

## Hardware
ELM327 USB or Bluetooth adapter connected to vehicle OBD-II port (under dash).
Runs on Raspberry Pi or any Linux machine.

## Install
pip install -r requirements.txt
python pyobd.py

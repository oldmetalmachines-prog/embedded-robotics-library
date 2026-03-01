# Open SAE J1939 (C)

Source: https://github.com/DanielMartensson/Open-SAE-J1939
Author: Daniel Mårtensson
License: MIT
Language: ANSI C (C89) - MISRA C compatible

## Purpose
SAE J1939 protocol for farm and industrial vehicles — tractors, combines,
trucks, excavators, valves, engines, actuators. Any heavy equipment using CAN bus.

## Targets
STM32, Arduino, AVR, PIC, ESP32, Linux SocketCAN, Raspberry Pi

## Key Folders
- `Examples/` — Working examples for different hardware targets
- `Open_SAE_J1939/` — Core protocol library
- `SAE_J1939/` — SAE J1939 message definitions
- `Documentation/` — Protocol documentation

## Hardware Target
Any node in the rack with a CAN interface, or directly on ESP32/STM32.

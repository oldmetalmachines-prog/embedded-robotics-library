# Yaskawa EtherCAT Servo Control (C)

Source: https://github.com/CalvinHsu1223/IGH-EtherCAT-motor-control-sample
License: See original repository
Language: C

## Files

- `main_yas.c` — Basic Yaskawa Sigma-7 EtherCAT control via IGH master
- `main_yas2.c` — Extended Yaskawa control example
- `main_yas_dc.c` — Yaskawa with distributed clocks (real-time sync)
- `main_igh_ethercat.c` — Generic IGH EtherCAT master application

## Requirements
- IGH EtherCAT master installed on Linux
- Yaskawa Sigma-7 SERVOPACK with EtherCAT option (SGD7S)
- Dedicated Ethernet port for EtherCAT network

## Hardware Target
Raspberry Pi 5 or Jetson Orin with preempt-RT kernel for real-time performance.

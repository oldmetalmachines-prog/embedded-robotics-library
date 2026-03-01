# Multi-Node Distributed ROS2 Setup

Distributed ROS2 across Jetson Orin Nano, Jetson AGX, and Raspberry Pi 5 rack.

## Network Topology
```
192.168.50.0/24
├── 192.168.50.90  - Jetson Orin Nano Super  (AI/vision, GPIO, CAN)
├── 192.168.50.xx  - Jetson AGX              (heavy compute, orchestration)
├── 192.168.50.10  - Pi 5 Node 1             (sensor I/O)
├── 192.168.50.11  - Pi 5 Node 2             (sensor I/O)
├── 192.168.50.12  - Pi 5 Node 3             (motor control)
└── 192.168.50.13  - Pi 5 Node 4             (CAN bridge)
```

## ROS2 Distribution

Use **ROS2 Jazzy** (Ubuntu 24.04) on Pi nodes and AGX.
Use **ROS2 Humble** on Jetson Orin Nano (JetPack 6.x compatibility).

## DDS Configuration

All nodes must share the same ROS_DOMAIN_ID:
```bash
echo 'export ROS_DOMAIN_ID=42' >> ~/.bashrc
echo 'export ROS_LOCALHOST_ONLY=0' >> ~/.bashrc
source ~/.bashrc
```

## Node Role Assignments

### Jetson Orin Nano (192.168.50.90)
- Isaac ROS inference nodes (object detection, visual SLAM)
- GPIO publisher nodes (sensor triggers, status LEDs)
- CAN bus bridge (ros2_socketcan → can0)

### Jetson AGX
- Navigation stack (Nav2)
- MoveIt2 motion planning
- Map server / SLAM coordinator
- rosbag recording

### Raspberry Pi 5 Nodes
- Modbus RTU polling (Leadshine servos)
- EtherCAT master (ethercrab)
- GPIO hardware triggers
- Sensor data publishers

## Verifying Multi-Node Communication
```bash
ros2 node list        # Should show nodes from all machines
ros2 topic list       # Should show all topics across network
ros2 doctor           # Diagnose DDS/network issues
```

## Tips

- Run `ros2 daemon stop && ros2 daemon start` if nodes are not discovering each other
- For bandwidth-heavy topics (images, pointclouds), consider CycloneDDS over FastDDS
- Jetson Orin: set power mode to MAXN for full performance (nvpmodel -m 0)
- NEVER run apt upgrade on Jetson Orin (192.168.50.90) - will break JetPack

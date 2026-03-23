# Robotic Arm Projects

Open-source robotic arm libraries and SDKs, included for reference and use in embedded/lab projects.
All projects are third-party — see `../THIRD_PARTY_NOTICES.md` for attribution.

## Projects

### rs-opw-kinematics (Rust)
- **Source**: https://github.com/bourumir-wyngs/rs-opw-kinematics
- **License**: BSD-3-Clause
- **Description**: Analytical inverse/forward kinematics for 6-axis OPW robots (parallel base, spherical wrist)
- **Compile status**: VERIFIED on Jetson Orin Nano (aarch64, Rust 1.94)
- **Build note**: Use `default-features = false` to exclude the bevy visualization layer (wayland/GUI deps)
  ```toml
  rs-opw-kinematics = { version = "1.8.16", default-features = false }
  ```

### DynamixelSDK (C / C++ / Python)
- **Source**: https://github.com/ROBOTIS-GIT/DynamixelSDK
- **License**: Apache-2.0
- **Description**: Official SDK for Dynamixel servo motors; Protocol 1.0 and 2.0; serial communication
- **Compile status**: VERIFIED (C) on Jetson Orin Nano (aarch64, gcc 11.4)
- **Build**: `cd c && make`

### pymycobot (Python)
- **Source**: https://github.com/elephantrobotics/pymycobot
- **License**: MIT
- **Description**: Python serial API for MyCobot/MechArm robotic arms; packet encoding/decoding
- **Compile status**: VERIFIED on Jetson Orin Nano (Python 3.10)
- **Install**: `pip install pymycobot`

### openrr (Rust workspace)
- **Source**: https://github.com/openrr/openrr
- **License**: Apache-2.0
- **Description**: Open Robot Rust framework; path planning, collision avoidance, multi-robot support
- **Compile status**: PARTIAL — core `k` kinematics crate and `openrr-planner` verified; `arci-ros`/`arci-ros2` require ROS1/2 system environment
- **Build note**: To use the kinematics core without ROS:
  ```toml
  k = { version = "0.32", default-features = false }
  ```

### interbotix_ros_manipulators (ROS2 / Python / C++)
- **Source**: https://github.com/Interbotix/interbotix_ros_manipulators
- **License**: BSD-2-Clause
- **Description**: ROS2 packages for Interbotix arms (WidowX, ReactorX, etc.); MoveIt integration
- **Compile status**: REQUIRES ROS2 (Humble/Iron) + colcon build system
- **Build**: ROS2 workspace only — `colcon build --packages-select interbotix_xs_sdk`

### moveo_ros (ROS1 / C++)
- **Source**: https://github.com/4ndreas/BionicArm
- **License**: MIT
- **Description**: ROS1 packages for the open-source Moveo 3D-printed robotic arm
- **Compile status**: REQUIRES ROS1 (Noetic) + catkin build system
- **Build**: catkin workspace only — `catkin_make`

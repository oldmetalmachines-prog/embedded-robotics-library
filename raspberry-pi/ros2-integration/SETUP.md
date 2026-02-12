# ROS2 Integration Setup Guide

Complete guide for integrating Rust robotics code with ROS2 on Raspberry Pi.

## Prerequisites

### 1. Install ROS2 Jazzy on Ubuntu 24.04
```bash
# Add ROS2 repository
sudo apt update
sudo apt install software-properties-common
sudo add-apt-repository universe

sudo curl -sSL https://raw.githubusercontent.com/ros/rosdistro/master/ros.asc | sudo apt-key add -
sudo sh -c 'echo "deb http://packages.ros.org/ros2/ubuntu $(lsb_release -sc) main" > /etc/apt/sources.list.d/ros2-latest.list'

# Install ROS2 Jazzy
sudo apt update
sudo apt install ros-jazzy-desktop

# Source ROS2
echo "source /opt/ros/jazzy/setup.bash" >> ~/.bashrc
source ~/.bashrc
```

### 2. Install r2r Rust Bindings
```bash
# Install dependencies
sudo apt install -y \
    libclang-dev \
    python3-colcon-common-extensions

# The r2r crate will be added via Cargo.toml
```

### 3. Set Environment Variables
```bash
# Add to ~/.bashrc
export AMENT_PREFIX_PATH=/opt/ros/jazzy
export CMAKE_PREFIX_PATH=/opt/ros/jazzy

source ~/.bashrc
```

## Project Structure
```
rpi-ros2-robot/
├── Cargo.toml
├── src/
│   ├── main.rs
│   ├── nodes/
│   │   ├── imu_publisher.rs
│   │   ├── motor_controller.rs
│   │   ├── odometry.rs
│   │   └── sensor_fusion.rs
│   ├── drivers/
│   │   ├── motors.rs
│   │   └── sensors.rs
│   └── utils/
│       ├── kinematics.rs
│       └── transforms.rs
└── launch/
    └── robot.launch.py
```

## Common ROS2 Topics for Robotics

### Sensors
- `/imu/data` - sensor_msgs/Imu
- `/scan` - sensor_msgs/LaserScan
- `/camera/image_raw` - sensor_msgs/Image
- `/odom` - nav_msgs/Odometry

### Control
- `/cmd_vel` - geometry_msgs/Twist
- `/joint_states` - sensor_msgs/JointState

### Navigation
- `/map` - nav_msgs/OccupancyGrid
- `/goal_pose` - geometry_msgs/PoseStamped

## Building and Running
```bash
# Build
cargo build --release

# Run IMU publisher
cargo run --bin imu_publisher

# In another terminal, verify
ros2 topic list
ros2 topic echo /imu/data

# Run motor controller
cargo run --bin motor_controller

# Test motors
ros2 topic pub /cmd_vel geometry_msgs/Twist "{linear: {x: 0.5}, angular: {z: 0.0}}"
```

## Integration with Jetson Orin

### Architecture
- **Raspberry Pi 5**: Motor control, sensor reading (low-level)
- **Jetson Orin Nano**: Navigation, SLAM, vision processing (high-level)
- **ESP32-P4**: Real-time sensor fusion, emergency stop

### Communication
All communicate via ROS2 topics over the same network.
```bash
# On Raspberry Pi
export ROS_DOMAIN_ID=0
export ROS_LOCALHOST_ONLY=0

# On Jetson
export ROS_DOMAIN_ID=0
export ROS_LOCALHOST_ONLY=0
```

## Debugging
```bash
# Check ROS2 communication
ros2 doctor

# Monitor topics
ros2 topic list
ros2 topic hz /imu/data
ros2 topic bw /imu/data

# View node graph
rqt_graph
```

## Performance Tips

1. **Use Fast DDS** (default in Jazzy)
2. **QoS Settings** - Use BEST_EFFORT for sensor data
3. **Separate executors** - One per core
4. **Zero-copy** - Use shared memory for large messages

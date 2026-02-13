# Jetson Orin Nano Super Platform Profile

**Official Name:** NVIDIA Jetson Orin Nano Super Developer Kit  
**Module:** Jetson Orin Nano Super (16GB variant available)  
**GPU:** NVIDIA Ampere with 1024 CUDA cores, 32 Tensor cores  
**CPU:** 6-core ARM Cortex-A78AE @ 1.5 GHz  
**RAM:** 8GB LPDDR5 (or 16GB variant)  
**Storage:** microSD slot (recommend 128GB+ UHS-I)  
**AI Performance:** 67 TOPS INT8  
**Power:** 15W / 25W modes  

**Best for:** Computer vision, AI inference, autonomous robots, ROS2 navigation, SLAM

---

## Quick Reference

### GPIO Header (40-pin Jetson GPIO)

Compatible with Raspberry Pi HAT form factor.

**Default I2C Buses:**
- **I2C Bus 1:** GPIO3 (SDA), GPIO5 (SCL) - `/dev/i2c-1`
- **I2C Bus 7:** GPIO27 (SDA), GPIO28 (SCL) - `/dev/i2c-7`
- **I2C Bus 8:** GPIO7 (SDA), GPIO29 (SCL) - `/dev/i2c-8`

**Default SPI:**
- **SPI0:** GPIO19 (MOSI), GPIO21 (MISO), GPIO23 (SCLK), GPIO24 (CS0)
- Supported but not commonly used (prefer I2C/UART for sensors)

**Default UART:**
- **UART1:** GPIO8 (TX), GPIO10 (RX) - `/dev/ttyTHS0`
- **Debug UART:** Accessible via micro-USB (console)

**PWM:**
- GPIO12, GPIO13, GPIO18, GPIO33 support hardware PWM
- All GPIO can do software PWM (less precise)

**Power:**
- **5V:** Pins 2, 4 (output from board)
- **3.3V:** Pins 1, 17 (output, max 50mA)
- **GND:** Pins 6, 9, 14, 20, 25, 30, 34, 39

---

## Jetson-Specific Features

### CUDA & TensorRT
- **CUDA Cores:** 1024 (Ampere architecture)
- **Tensor Cores:** 32 (for AI acceleration)
- **TensorRT:** Optimized inference engine
- **cuDNN:** Deep learning primitives

### Camera Interfaces
- **CSI Cameras:** 2x MIPI CSI-2 (22-pin connectors)
  - Supports up to 2x cameras simultaneously
  - Max resolution: 4K @ 30fps per camera
  - Compatible: IMX219, IMX477 (Raspberry Pi cameras work!)
- **USB Cameras:** Multiple via USB 3.0 ports

### Video Encode/Decode
- **H.264/H.265:** Hardware encode/decode
- **4K Support:** Up to 4K @ 30fps encode
- **Multiple Streams:** Can handle several video streams simultaneously

### Deep Learning Frameworks
- **PyTorch:** Full support with CUDA
- **TensorFlow:** With GPU acceleration
- **ONNX Runtime:** For model deployment
- **DeepStream:** Video analytics SDK

---

## Rust Development on Jetson

### System Setup
```bash
# Update system
sudo apt update && sudo apt upgrade -y

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Install development tools
sudo apt install -y build-essential cmake pkg-config \
  libssl-dev git curl wget

# For CUDA development (optional)
sudo apt install -y cuda-toolkit-11-4
```

### Rust for GPIO (Using sysfs_gpio)
```bash
# Add Rust target (native ARM)
rustup target add aarch64-unknown-linux-gnu

# GPIO libraries
# Option 1: sysfs_gpio (simple, portable)
# Option 2: gpio-cdev (modern, recommended)
# Option 3: rppal (if Raspberry Pi compatibility needed)
```

---

## Project Setup

### Basic GPIO Project
```toml
[package]
name = "jetson-gpio-example"
version = "0.1.0"
edition = "2021"

[dependencies]
gpio-cdev = "0.6"
embedded-hal = "1.0"
anyhow = "1.0"

[profile.release]
opt-level = 3
lto = true
```

### Computer Vision Project
```toml
[package]
name = "jetson-vision-example"
version = "0.1.0"
edition = "2021"

[dependencies]
opencv = { version = "0.88", features = ["cuda"] }
image = "0.24"
ndarray = "0.15"

# For AI inference
onnxruntime = { version = "0.0.14", features = ["cuda"] }

# For ROS2 integration
r2r = "0.8"
```

---

## GPIO Pin Map (40-pin Header)
```
              Jetson Orin Nano GPIO Header
         ╔════════════════════════════════╗
   3.3V  ║ 1  ●                      ● 2  ║ 5V
  GPIO3  ║ 3  ●  (I2C1_SDA)          ● 4  ║ 5V
  GPIO5  ║ 5  ●  (I2C1_SCL)          ● 6  ║ GND
  GPIO7  ║ 7  ●  (I2C8_SDA)          ● 8  ║ GPIO8  (UART1_TX)
    GND  ║ 9  ●                      ● 10 ║ GPIO10 (UART1_RX)
 GPIO11  ║ 11 ●                      ● 12 ║ GPIO12 (PWM)
 GPIO13  ║ 13 ●  (PWM)               ● 14 ║ GND
 GPIO15  ║ 15 ●                      ● 16 ║ GPIO16
   3.3V  ║ 17 ●                      ● 18 ║ GPIO18 (PWM)
 GPIO19  ║ 19 ●  (SPI0_MOSI)         ● 20 ║ GND
 GPIO21  ║ 21 ●  (SPI0_MISO)         ● 22 ║ GPIO22
 GPIO23  ║ 23 ●  (SPI0_SCLK)         ● 24 ║ GPIO24 (SPI0_CS0)
    GND  ║ 25 ●                      ● 26 ║ GPIO26 (SPI0_CS1)
 GPIO27  ║ 27 ●  (I2C7_SDA)          ● 28 ║ GPIO28 (I2C7_SCL)
 GPIO29  ║ 29 ●  (I2C8_SCL)          ● 30 ║ GND
 GPIO31  ║ 31 ●                      ● 32 ║ GPIO32
 GPIO33  ║ 33 ●  (PWM)               ● 34 ║ GND
 GPIO35  ║ 35 ●                      ● 36 ║ GPIO36
 GPIO37  ║ 37 ●                      ● 38 ║ GPIO38
    GND  ║ 39 ●                      ● 40 ║ GPIO40
         ╚════════════════════════════════╝
```

**Note:** GPIO numbering matches Raspberry Pi for compatibility with HATs.

---

## Common Configurations

### I2C Sensor (e.g., MPU6050, BME280)
```
Sensor  →  Jetson
VCC     →  3.3V (pin 1 or 17) - Max 50mA!
GND     →  GND (pin 6)
SDA     →  GPIO3 (pin 3) - I2C Bus 1
SCL     →  GPIO5 (pin 5) - I2C Bus 1
```

**Test I2C:**
```bash
# Install i2c-tools
sudo apt install i2c-tools

# Scan bus 1
i2cdetect -y -r 1

# Should show device at 0x68 (MPU6050) or 0x76 (BME280)
```

### CSI Camera (IMX219)
```
Camera Module → Jetson
Connect to CAM0 or CAM1 port (22-pin ribbon cable)

Verify:
v4l2-ctl --list-devices
# Should show /dev/video0
```

### USB Camera
```
Camera → USB 3.0 port

Test:
ls /dev/video*
# Should show /dev/video0 or /dev/video1

# Test with GStreamer:
gst-launch-1.0 v4l2src device=/dev/video0 ! autovideosink
```

### Motor Driver (L298N)
```
L298N   →  Jetson
IN1     →  GPIO11 (pin 11)
IN2     →  GPIO13 (pin 13)
IN3     →  GPIO15 (pin 15)
IN4     →  GPIO16 (pin 16)
ENA     →  GPIO12 (pin 12) - PWM
ENB     →  GPIO18 (pin 18) - PWM
GND     →  GND
VCC     →  External 5V power (NOT from Jetson!)
```

---

## Power Requirements

- **Input:** 5V via barrel jack (5.5mm x 2.5mm) or USB-C
- **Recommended Supply:** 5V/4A (20W) minimum
- **Power Modes:**
  - 10W: Low power mode
  - 15W: Default mode (balanced)
  - 25W: Max performance mode
- **Current Draw:**
  - Idle: ~2A @ 5V (10W)
  - Under load: ~4-5A @ 5V (20-25W)
  - With peripherals: Plan for 6A supply

**⚠️  CRITICAL:** Do NOT power motors or servos from Jetson GPIO!  
Always use external power supply for actuators.

---

## Jetson Software Stack

### JetPack SDK (Pre-installed on Developer Kit)

Current: **JetPack 6.0** (based on Ubuntu 22.04)

Includes:
- **CUDA Toolkit** - GPU programming
- **cuDNN** - Deep learning acceleration
- **TensorRT** - Inference optimization
- **VPI** - Vision programming interface
- **DeepStream** - Video analytics
- **Multimedia API** - Camera/video encoding

### Check JetPack Version
```bash
cat /etc/nv_tegra_release
# Or:
sudo apt-cache show nvidia-jetpack
```

---

## Computer Vision with Rust

### OpenCV with CUDA
```bash
# Install OpenCV with CUDA support
sudo apt install -y libopencv-dev libopencv-contrib-dev

# Verify CUDA support
python3 -c "import cv2; print(cv2.cuda.getCudaEnabledDeviceCount())"
# Should return 1 (or more)
```

**Rust OpenCV Example:**
```rust
use opencv::{
    prelude::*,
    videoio,
    core,
    highgui,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut cam = videoio::VideoCapture::new(0, videoio::CAP_V4L2)?;
    let mut frame = Mat::default();
    
    loop {
        cam.read(&mut frame)?;
        highgui::imshow("Camera", &frame)?;
        
        if highgui::wait_key(1)? > 0 { break; }
    }
    Ok(())
}
```

---

## ROS2 on Jetson

### Install ROS2 Humble
```bash
# Add ROS2 repository
sudo apt update
sudo apt install software-properties-common
sudo add-apt-repository universe

# Install ROS2 Humble
sudo apt update
sudo apt install -y ros-humble-desktop

# Source ROS2
echo "source /opt/ros/humble/setup.bash" >> ~/.bashrc
source ~/.bashrc

# Install development tools
sudo apt install -y python3-colcon-common-extensions
```

### Rust ROS2 Integration

See: `raspberry-pi/ros2-integration/SETUP.md` (same process)
```toml
[dependencies]
r2r = "0.8"
tokio = { version = "1", features = ["full"] }
```

---

## AI Inference

### ONNX Runtime with CUDA
```bash
# Install ONNX Runtime
pip3 install onnxruntime-gpu

# Verify CUDA support
python3 -c "import onnxruntime as ort; print(ort.get_available_providers())"
# Should include 'CUDAExecutionProvider'
```

**Rust ONNX Example Structure:**
```toml
[dependencies]
onnxruntime = { version = "0.0.14", features = ["cuda"] }
ndarray = "0.15"
image = "0.24"
```

---

## Performance Optimization

### Power Mode
```bash
# Check current mode
sudo nvpmodel -q

# Set to max performance (25W)
sudo nvpmodel -m 0

# Set to balanced (15W)
sudo nvpmodel -m 1

# Monitor power
sudo tegrastats
```

### CPU Frequency
```bash
# Set max CPU frequency
sudo jetson_clocks

# Check current frequency
cat /sys/devices/system/cpu/cpu*/cpufreq/scaling_cur_freq
```

---

## Troubleshooting

### "Permission denied" on GPIO
```bash
# Add user to gpio group
sudo usermod -a -G gpio $USER
sudo usermod -a -G i2c $USER

# Logout and login
```

---

### "CUDA out of memory"

**Solutions:**
- Reduce batch size in AI models
- Use TensorRT optimization
- Switch to 16GB variant
- Monitor with `tegrastats`

---

### "Camera not detected"

**Check:**
```bash
# List cameras
ls /dev/video*
v4l2-ctl --list-devices

# For CSI cameras
dmesg | grep -i camera

# Verify cable connection (blue stripe up on CAM0/CAM1)
```

---

### "I2C device not found"
```bash
# List I2C buses
i2cdetect -l

# Scan bus 1
i2cdetect -y -r 1

# Check permissions
ls -l /dev/i2c-*

# Add to i2c group if needed
sudo usermod -a -G i2c $USER
```

---

## Jetson vs Raspberry Pi

| Feature | Jetson Orin Nano Super | Raspberry Pi 5 |
|---------|----------------------|----------------|
| CPU | 6x Cortex-A78AE @ 1.5GHz | 4x Cortex-A76 @ 2.4GHz |
| GPU | 1024 CUDA cores | VideoCore VII (no CUDA) |
| AI Performance | 67 TOPS INT8 | ~2-3 TOPS (via NPU) |
| RAM | 8GB/16GB LPDDR5 | 4GB/8GB LPDDR4X |
| Computer Vision | Excellent (CUDA, TensorRT) | Good (CPU-based) |
| Power | 15-25W | 5-10W |
| Price | ~$250 | ~$60 |
| Best For | AI, vision, autonomous | General compute, GPIO |

**Use Jetson for:** Heavy AI inference, real-time vision, autonomous navigation  
**Use Pi for:** Lightweight compute, GPIO control, cost-sensitive projects

---

## Typical Robot Architecture
```
Jetson Orin Nano Super (Main Compute)
├── Computer Vision (CUDA-accelerated)
│   ├── Object detection
│   ├── Semantic segmentation
│   └── Visual SLAM
├── ROS2 Navigation Stack
│   ├── Path planning
│   ├── Localization (AMCL)
│   └── Obstacle avoidance
└── ESP32-P4 (Real-time Control) via UDP/ROS2
    ├── Sensor fusion (IMU, encoders)
    ├── Motor PWM control
    └── Emergency stop
```

---

## Example Use Cases

1. **Autonomous Rover**
   - Jetson: Vision, navigation, path planning
   - ESP32-P4: Motor control, sensor fusion
   - Pi 5: Sensor aggregation (optional)

2. **Object Detection Robot**
   - Jetson: Run YOLO/SSD object detection
   - Publish detections to ROS2
   - ESP32 acts on detections (move toward/away)

3. **SLAM Mapping**
   - Jetson: Visual SLAM (ORB-SLAM3)
   - LiDAR processing
   - Map building and localization

4. **Gesture Control**
   - Camera captures hand gestures
   - Jetson runs pose estimation
   - Controls robot via ROS2 messages

---

## Datasheets and Resources

- [Jetson Orin Nano Developer Kit User Guide](https://developer.nvidia.com/embedded/learn/jetson-orin-nano-devkit-user-guide)
- [Jetson Linux Documentation](https://docs.nvidia.com/jetson/archives/r36.3/DeveloperGuide/)
- [CUDA Toolkit Documentation](https://docs.nvidia.com/cuda/)
- [TensorRT Documentation](https://docs.nvidia.com/deeplearning/tensorrt/)
- [JetPack SDK](https://developer.nvidia.com/embedded/jetpack)

---

## Library Examples for Jetson

See:
- `jetson-orin/examples/` - Coming soon!
- `raspberry-pi/ros2-integration/` - ROS2 examples work on Jetson

---

## Changelog

- 2026-02-13: Initial Jetson Orin Nano Super platform profile
- Pin maps, GPIO, I2C, camera interfaces
- CUDA/TensorRT notes for AI acceleration
- ROS2 integration guidance
- Power optimization tips

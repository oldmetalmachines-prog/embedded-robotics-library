//! ROS2 IMU Publisher
//!
//! Publishes IMU data to /imu/data topic
//! Message type: sensor_msgs/Imu

use anyhow::Result;
use r2r::sensor_msgs::msg::Imu;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<()> {
    let ctx = r2r::Context::create()?;
    let mut node = r2r::Node::create(ctx, "imu_publisher", "")?;
    
    let publisher = node.create_publisher::<Imu>(
        "/imu/data", 
        r2r::QosProfile::default()
    )?;
    
    println!("Publishing to /imu/data at 100Hz");

    let mut timer = tokio::time::interval(Duration::from_millis(10));

    loop {
        timer.tick().await;
        
        let msg = Imu::default();
        publisher.publish(&msg)?;
    }
}

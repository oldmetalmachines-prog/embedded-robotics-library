//! ROS2 Motor Controller
//!
//! Subscribes to /cmd_vel and controls motors

use anyhow::Result;
use r2r::geometry_msgs::msg::Twist;

#[tokio::main]
async fn main() -> Result<()> {
    let ctx = r2r::Context::create()?;
    let mut node = r2r::Node::create(ctx, "motor_controller", "")?;
    
    let mut subscriber = node.subscribe::<Twist>(
        "/cmd_vel", 
        r2r::QosProfile::default()
    )?;
    
    println!("Listening to /cmd_vel");

    loop {
        if let Some(msg) = subscriber.next().await {
            println!("Linear: {:.2}, Angular: {:.2}", 
                msg.linear.x, msg.angular.z);
        }
    }
}

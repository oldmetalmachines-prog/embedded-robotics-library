use std::net::UdpSocket;
use serde::Deserialize;
use chrono::Local;
use colored::*;

// Telemetry v1 Schema Structs
#[derive(Deserialize, Debug)]
struct TelemetryHeader {
    device_id: String,
    timestamp: u64,
    sequence: u32,
    message_type: String,
}

#[derive(Deserialize, Debug)]
struct ImuData {
    accel_x: f32,
    accel_y: f32,
    accel_z: f32,
    gyro_x: f32,
    gyro_y: f32,
    gyro_z: f32,
    temperature_c: f32,
}

#[derive(Deserialize, Debug)]
struct TelemetryData {
    imu: Option<ImuData>,
}

#[derive(Deserialize, Debug)]
struct TelemetryMessage {
    header: TelemetryHeader,
    telemetry: Option<TelemetryData>,
}

fn main() -> std::io::Result<()> {
    println!("{}", "=== UDP Telemetry Receiver ===".bright_cyan().bold());
    println!("Listening on 0.0.0.0:8888 for telemetry packets\n");

    // Bind to all interfaces on port 8888
    let socket = UdpSocket::bind("0.0.0.0:8888")?;
    println!("{}", "✓ UDP socket bound successfully".green());
    println!("{}", "Waiting for telemetry data...\n".yellow());

    let mut buffer = [0u8; 2048];
    let mut last_seq: Option<u32> = None;
    let mut packet_count = 0u64;
    let mut error_count = 0u64;

    loop {
        match socket.recv_from(&mut buffer) {
            Ok((size, src)) => {
                packet_count += 1;

                // Parse JSON
                match serde_json::from_slice::<TelemetryMessage>(&buffer[..size]) {
                    Ok(msg) => {
                        let now = Local::now().format("%H:%M:%S%.3f");

                        // Check for packet loss
                        if let Some(prev_seq) = last_seq {
                            let expected = prev_seq.wrapping_add(1);
                            if msg.header.sequence != expected && msg.header.sequence != 0 {
                                let lost = if msg.header.sequence > expected {
                                    msg.header.sequence - expected
                                } else {
                                    1  // Sequence wrapped
                                };
                                println!("{} {} packets", 
                                    "⚠️  Lost".yellow(),
                                    lost);
                            }
                        }
                        last_seq = Some(msg.header.sequence);

                        // Display header
                        println!("{} {} {} seq={} from {}",
                            format!("[{}]", now).bright_black(),
                            msg.header.device_id.bright_blue(),
                            msg.header.message_type.bright_green(),
                            msg.header.sequence.to_string().bright_white(),
                            src.to_string().bright_black()
                        );

                        // Display IMU data if present
                        if let Some(telemetry) = msg.telemetry {
                            if let Some(imu) = telemetry.imu {
                                println!("  {} Accel: ({:6.2}, {:6.2}, {:6.2}) m/s²",
                                    "📊".bright_yellow(),
                                    imu.accel_x, imu.accel_y, imu.accel_z
                                );
                                println!("  {} Gyro:  ({:6.2}, {:6.2}, {:6.2}) deg/s",
                                    "🔄".bright_cyan(),
                                    imu.gyro_x, imu.gyro_y, imu.gyro_z
                                );
                                println!("  {} Temp:  {:.1}°C",
                                    "🌡️ ".bright_magenta(),
                                    imu.temperature_c
                                );
                            }
                        }

                        // Stats every 100 packets
                        if packet_count % 100 == 0 {
                            let success_rate = 
                                (packet_count as f64 / (packet_count + error_count) as f64) * 100.0;
                            println!("{} Received: {}, Errors: {}, Success: {:.1}%\n",
                                "📈".bright_white(),
                                packet_count.to_string().bright_green(),
                                error_count.to_string().bright_red(),
                                success_rate
                            );
                        }
                    }
                    Err(e) => {
                        error_count += 1;
                        eprintln!("{} JSON parse error: {} (size: {} bytes)",
                            "❌".bright_red(),
                            e,
                            size
                        );
                        // Show raw data for debugging
                        if let Ok(s) = std::str::from_utf8(&buffer[..size]) {
                            eprintln!("   Raw: {}", s);
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!("{} Socket error: {}", "❌".bright_red(), e);
            }
        }
    }
}

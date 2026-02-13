#![no_std]
#![no_main]

extern crate alloc;

use esp_backtrace as _;
use esp_hal::{
    clock::ClockControl,
    peripherals::Peripherals,
    prelude::*,
    rng::Rng,
    timer::systimer::SystemTimer,
};
use esp_println::println;
use esp_wifi::{
    initialize,
    wifi::{
        ClientConfiguration, Configuration, WifiController, WifiDevice, WifiEvent, WifiStaDevice,
        WifiState,
    },
    EspWifiInitFor,
};
use smoltcp::{
    iface::{Config, Interface, SocketSet},
    socket::udp,
    wire::{IpAddress, IpEndpoint, Ipv4Address},
};
use serde::Serialize;
use heapless::String;

// WiFi credentials - CHANGE THESE!
const SSID: &str = "YOUR_WIFI_SSID";
const PASSWORD: &str = "YOUR_WIFI_PASSWORD";

// UDP target - CHANGE THIS to your Raspberry Pi IP
const TARGET_IP: [u8; 4] = [192, 168, 1, 100];  // Example: 192.168.1.100
const TARGET_PORT: u16 = 8888;

// Telemetry v1 Schema Structs
#[derive(Serialize)]
struct TelemetryHeader {
    device_id: String<32>,
    timestamp: u64,
    sequence: u32,
    message_type: String<16>,
}

#[derive(Serialize)]
struct ImuData {
    accel_x: f32,
    accel_y: f32,
    accel_z: f32,
    gyro_x: f32,
    gyro_y: f32,
    gyro_z: f32,
    temperature_c: f32,
}

#[derive(Serialize)]
struct TelemetryData {
    imu: ImuData,
}

#[derive(Serialize)]
struct TelemetryMessage {
    header: TelemetryHeader,
    telemetry: TelemetryData,
}

#[entry]
fn main() -> ! {
    // Initialize
    let peripherals = Peripherals::take();
    let system = peripherals.SYSTEM.split();
    let clocks = ClockControl::max(system.clock_control).freeze();

    // Setup heap
    esp_alloc::heap_allocator!(72 * 1024);

    let timer = SystemTimer::new(peripherals.SYSTIMER).alarm0;
    let init = initialize(
        EspWifiInitFor::Wifi,
        timer,
        Rng::new(peripherals.RNG),
        peripherals.RADIO_CLK,
        &clocks,
    )
    .unwrap();

    let wifi = peripherals.WIFI;
    let (wifi_interface, controller) =
        esp_wifi::wifi::new_with_mode(&init, wifi, WifiStaDevice).unwrap();

    // Network setup
    let mut socket_set_entries: [_; 3] = Default::default();
    let mut socket_set = SocketSet::new(&mut socket_set_entries[..]);

    let config = Config::new(smoltcp::wire::HardwareAddress::Ethernet(
        smoltcp::wire::EthernetAddress::from_bytes(&[0x02, 0x00, 0x00, 0x00, 0x00, 0x00]),
    ));

    let mut interface = Interface::new(config, &mut wifi_interface.device(), smoltcp::time::Instant::ZERO);

    // Create UDP socket
    let udp_rx_buffer = udp::PacketBuffer::new(
        vec![udp::PacketMetadata::EMPTY; 4],
        vec![0u8; 1024],
    );
    let udp_tx_buffer = udp::PacketBuffer::new(
        vec![udp::PacketMetadata::EMPTY; 4],
        vec![0u8; 1024],
    );
    let udp_socket = udp::Socket::new(udp_rx_buffer, udp_tx_buffer);
    let udp_handle = socket_set.add(udp_socket);

    println!("\n=== UDP Telemetry Sender ===");
    println!("Connecting to WiFi: {}", SSID);

    // WiFi connection
    let client_config = Configuration::Client(ClientConfiguration {
        ssid: SSID.try_into().unwrap(),
        password: PASSWORD.try_into().unwrap(),
        ..Default::default()
    });

    let res = controller.set_configuration(&client_config);
    println!("WiFi set configuration returned {:?}", res);

    controller.start().unwrap();
    println!("WiFi started");

    // Wait for connection
    loop {
        let res = controller.connect();
        match res {
            Ok(_) => {
                println!("WiFi connected!");
                break;
            }
            Err(e) => {
                println!("Failed to connect: {:?}", e);
                esp_hal::delay::Delay::new(&clocks).delay_ms(1000u32);
            }
        }
    }

    // Wait for IP
    println!("Waiting for IP...");
    loop {
        wifi_interface.poll_ingress();
        interface.poll(smoltcp::time::Instant::from_millis(
            esp_hal::time::now().duration_since_epoch().to_millis() as i64,
        ), &mut socket_set);

        if let Some(config) = interface.ipv4_config() {
            println!("Got IP: {}", config.address);
            break;
        }
    }

    println!("Sending telemetry to {}:{}", 
        Ipv4Address::from_bytes(&TARGET_IP), TARGET_PORT);

    // Main telemetry loop
    let mut seq = 0u32;
    let mut delay = esp_hal::delay::Delay::new(&clocks);

    loop {
        // Simulate sensor reading (replace with real MPU6050 later)
        let msg = TelemetryMessage {
            header: TelemetryHeader {
                device_id: String::from("esp32-test-01"),
                timestamp: esp_hal::time::now().duration_since_epoch().to_millis(),
                sequence: seq,
                message_type: String::from("telemetry"),
            },
            telemetry: TelemetryData {
                imu: ImuData {
                    accel_x: 0.02,
                    accel_y: 0.01,
                    accel_z: 9.81,
                    gyro_x: 0.0,
                    gyro_y: -0.5,
                    gyro_z: 0.1,
                    temperature_c: 23.5,
                },
            },
        };

        // Serialize to JSON
        let mut buffer = [0u8; 512];
        match serde_json_core::to_slice(&msg, &mut buffer) {
            Ok(json_len) => {
                let json_data = &buffer[..json_len];

                // Send UDP packet
                let socket = socket_set.get_mut::<udp::Socket>(udp_handle);
                let remote = IpEndpoint::new(
                    IpAddress::Ipv4(Ipv4Address::from_bytes(&TARGET_IP)),
                    TARGET_PORT,
                );

                match socket.send_slice(json_data, remote) {
                    Ok(_) => {
                        println!("Sent seq {} ({} bytes)", seq, json_len);
                    }
                    Err(e) => {
                        println!("Send error: {:?}", e);
                    }
                }
            }
            Err(e) => {
                println!("JSON error: {:?}", e);
            }
        }

        seq = seq.wrapping_add(1);

        // Poll network
        wifi_interface.poll_ingress();
        interface.poll(smoltcp::time::Instant::from_millis(
            esp_hal::time::now().duration_since_epoch().to_millis() as i64,
        ), &mut socket_set);

        delay.delay_ms(100u32);  // Send at 10 Hz
    }
}

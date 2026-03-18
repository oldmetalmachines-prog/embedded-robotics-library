use esp_idf_svc::mqtt::client::*;
use esp_idf_svc::wifi::*;
use esp_idf_sys as _;
use log::*;
use std::env;
use std::time::Duration;

fn main() -> anyhow::Result<()> {
    esp_idf_sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    let mqtt_user = env::var("MQTT_USER").unwrap_or_default();
    let mqtt_pass = env::var("MQTT_PASS").unwrap_or_default();
    let broker = "mqtt://192.168.50.1:1883";

    info!("Connecting to MQTT broker: {}", broker);

    let mqtt_config = MqttClientConfiguration {
        client_id: Some("esp32-telemetry-sender"),
        username: if !mqtt_user.is_empty() { Some(&mqtt_user) } else { None },
        password: if !mqtt_pass.is_empty() { Some(&mqtt_pass) } else { None },
        ..Default::default()
    };

    let (mut client, mut connection) = EspMqttClient::new(broker, &mqtt_config)?;

    std::thread::spawn(move || {
        while let Ok(event) = connection.next() {
            info!("MQTT Event: {:?}", event);
        }
    });

    loop {
        let payload = r#"{"status": "ok", "temp": 25.5}"#;
        client.publish("sigma/telemetry/esp32", QOS_0, false, payload.as_bytes())?;
        info!("Published telemetry");
        std::thread::sleep(Duration::from_secs(10));
    }
}

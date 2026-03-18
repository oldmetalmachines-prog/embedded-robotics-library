# Jetson PWM Servo

Control a hobby servo from a Jetson Orin using the Linux sysfs PWM interface.

This example is designed for rack-friendly automation:
- servo settings are configurable through environment variables
- the optional `sigma-rack` feature publishes job state to MQTT on the rack broker
- the optional `SIGMA_V4_STATUS_URL` hook can POST job status JSON to a rack-local HTTP endpoint

## Features

- configurable PWM chip and channel
- configurable servo angle, pulse range, and hold time
- safe validation of period and duty-cycle values before writing sysfs files
- optional MQTT publishing to `rack/status/{job_id}`
- optional HTTP status callback for rack-local services

## Environment variables

| Variable | Default | Purpose |
| --- | --- | --- |
| `JETSON_PWM_CHIP` | `0` | PWM chip under `/sys/class/pwm` |
| `JETSON_PWM_CHANNEL` | `0` | PWM channel number |
| `JETSON_PWM_PERIOD_NS` | `20000000` | PWM period in nanoseconds |
| `JETSON_SERVO_MIN_PULSE_NS` | `500000` | Servo minimum pulse width |
| `JETSON_SERVO_MAX_PULSE_NS` | `2500000` | Servo maximum pulse width |
| `JETSON_SERVO_ANGLE_DEG` | `90` | Target angle from 0 to 180 |
| `JETSON_SERVO_HOLD_MS` | `2000` | How long to hold the servo position |
| `JETSON_SERVO_DISABLE_AFTER_HOLD` | `true` | Disable PWM after the hold interval |
| `JETSON_PWM_SETTLE_MS` | `100` | Delay after enabling PWM |
| `SIGMA_NODE_NAME` | hostname | Rack node name included in status messages |
| `SERVO_JOB_ID` | generated | Job identifier used in MQTT topics |
| `JETSON_SERVO_MQTT_TOPIC` | `rack/status/{job_id}` | Override MQTT topic |
| `SIGMA_V4_STATUS_URL` | unset | Optional JSON status POST endpoint |
| `MQTT_HOST` | `192.168.50.1` | Rack MQTT broker host |
| `MQTT_PORT` | `1883` | Rack MQTT broker port |
| `MQTT_USER` | required with `sigma-rack` | MQTT username |
| `MQTT_PASS` | required with `sigma-rack` | MQTT password |

## Build

```bash
cargo check --manifest-path jetson-orin/examples/pwm-servo/Cargo.toml
cargo check --manifest-path jetson-orin/examples/pwm-servo/Cargo.toml --features sigma-rack
```

## Run

```bash
JETSON_PWM_CHIP=0 \
JETSON_PWM_CHANNEL=0 \
JETSON_SERVO_ANGLE_DEG=45 \
JETSON_SERVO_HOLD_MS=1500 \
cargo run --manifest-path jetson-orin/examples/pwm-servo/Cargo.toml
```

With rack integration enabled:

```bash
MQTT_USER=your-user \
MQTT_PASS=your-pass \
SIGMA_NODE_NAME=jetson \
SERVO_JOB_ID=servo-demo-001 \
cargo run --manifest-path jetson-orin/examples/pwm-servo/Cargo.toml --features sigma-rack
```

## Wiring notes

This project assumes you already have a PWM-capable Jetson header pin routed through the correct board pinmux and level shifting for your servo setup.

Typical hobby-servo wiring:
- signal wire to the selected PWM output
- ground shared between the servo supply and Jetson
- servo power from an external 5V supply, not directly from the Jetson header

## MQTT payload

When the `sigma-rack` feature is enabled, the project publishes JSON like this:

```json
{
  "node_name": "jetson",
  "job_id": "jetson-servo-1710000000",
  "state": "active",
  "detail": "Set pwmchip0 channel 0 to 45.0 degrees (1000000 ns duty)",
  "angle_deg": 45.0,
  "pwm_chip": 0,
  "pwm_channel": 0,
  "period_ns": 20000000,
  "duty_cycle_ns": 1000000,
  "timestamp_ms": 1710000000000
}
```

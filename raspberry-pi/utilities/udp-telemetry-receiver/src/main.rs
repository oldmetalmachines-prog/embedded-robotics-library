use std::env;
use std::fs::{File, OpenOptions};
use std::io::{self, Write};
use std::net::UdpSocket;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

fn now_ms() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis()
}

fn open_append(path: &str) -> io::Result<File> {
    OpenOptions::new().create(true).append(true).open(path)
}

fn ensure_csv_header(path: &str) -> io::Result<()> {
    let p = Path::new(path);
    let need_header = !p.exists() || p.metadata().map(|m| m.len()).unwrap_or(0) == 0;
    if need_header {
        let mut f = open_append(path)?;
        writeln!(f, "ts_ms,src_ip,src_port,len,payload_utf8")?;
    }
    Ok(())
}

fn main() -> io::Result<()> {
    // Args:
    // 1: bind addr (default 0.0.0.0:9000)
    // 2: jsonl path (default telemetry.jsonl)
    // 3: csv path (default telemetry.csv)
    // 4: mode: jsonl|csv|both (default both)
    let args: Vec<String> = env::args().collect();

    let bind_addr = args.get(1).map(String::as_str).unwrap_or("0.0.0.0:9000");
    let jsonl_path = args.get(2).map(String::as_str).unwrap_or("telemetry.jsonl");
    let csv_path = args.get(3).map(String::as_str).unwrap_or("telemetry.csv");
    let mode = args.get(4).map(String::as_str).unwrap_or("both");

    let write_jsonl = mode == "jsonl" || mode == "both";
    let write_csv = mode == "csv" || mode == "both";

    if write_csv {
        ensure_csv_header(csv_path)?;
    }

    let socket = UdpSocket::bind(bind_addr)?;
    eprintln!("udp-telemetry-receiver listening on {}", bind_addr);
    eprintln!("mode={}, jsonl={}, csv={}", mode, jsonl_path, csv_path);

    let mut buf = [0u8; 2048];

    loop {
        let (n, src) = socket.recv_from(&mut buf)?;
        let ts = now_ms();

        let payload = &buf[..n];
        let payload_utf8 = match std::str::from_utf8(payload) {
            Ok(s) => s.replace('\n', "\\n").replace('\r', "\\r"),
            Err(_) => "<non-utf8>".to_string(),
        };

        if write_jsonl {
            let mut f = open_append(jsonl_path)?;
            let payload_escaped = payload_utf8.replace('\\', "\\\\").replace('"', "\\\"");
            writeln!(
                f,
                "{{\"ts_ms\":{},\"src\":\"{}\",\"len\":{},\"payload_utf8\":\"{}\"}}",
                ts, src, n, payload_escaped
            )?;
        }

        if write_csv {
            let mut f = open_append(csv_path)?;
            writeln!(
                f,
                "{},{},{},{},\"{}\"",
                ts,
                src.ip(),
                src.port(),
                n,
                payload_utf8.replace('"', "\"\"")
            )?;
        }
    }
}

use std::env;
use std::fs::{File, OpenOptions};
use std::io::{self, BufWriter, Write};
use std::net::UdpSocket;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Mode {
    Jsonl,
    Csv,
    Both,
}

impl Mode {
    fn parse(raw: &str) -> io::Result<Self> {
        match raw {
            "jsonl" => Ok(Self::Jsonl),
            "csv" => Ok(Self::Csv),
            "both" => Ok(Self::Both),
            other => Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("invalid mode '{other}', expected one of: jsonl, csv, both"),
            )),
        }
    }

    fn writes_jsonl(self) -> bool {
        matches!(self, Self::Jsonl | Self::Both)
    }

    fn writes_csv(self) -> bool {
        matches!(self, Self::Csv | Self::Both)
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Config {
    bind_addr: String,
    jsonl_path: String,
    csv_path: String,
    mode: Mode,
    buffer_size: usize,
    max_packets: Option<usize>,
}

impl Config {
    fn from_args<I>(args: I) -> io::Result<Self>
    where
        I: IntoIterator<Item = String>,
    {
        let mut bind_addr = String::from("0.0.0.0:9000");
        let mut jsonl_path = String::from("telemetry.jsonl");
        let mut csv_path = String::from("telemetry.csv");
        let mut mode = Mode::Both;
        let mut buffer_size = 2048usize;
        let mut max_packets = None;

        let mut iter = args.into_iter();
        let _program = iter.next();

        while let Some(arg) = iter.next() {
            match arg.as_str() {
                "--bind" => bind_addr = next_value(&mut iter, "--bind")?,
                "--jsonl" => jsonl_path = next_value(&mut iter, "--jsonl")?,
                "--csv" => csv_path = next_value(&mut iter, "--csv")?,
                "--mode" => mode = Mode::parse(&next_value(&mut iter, "--mode")?)?,
                "--buffer-size" => {
                    let raw = next_value(&mut iter, "--buffer-size")?;
                    buffer_size = raw.parse::<usize>().map_err(|_| {
                        io::Error::new(
                            io::ErrorKind::InvalidInput,
                            format!("invalid --buffer-size '{raw}', expected a positive integer"),
                        )
                    })?;
                    if buffer_size == 0 {
                        return Err(io::Error::new(
                            io::ErrorKind::InvalidInput,
                            "--buffer-size must be > 0",
                        ));
                    }
                }
                "--max-packets" => {
                    let raw = next_value(&mut iter, "--max-packets")?;
                    let parsed = raw.parse::<usize>().map_err(|_| {
                        io::Error::new(
                            io::ErrorKind::InvalidInput,
                            format!("invalid --max-packets '{raw}', expected a positive integer"),
                        )
                    })?;
                    if parsed == 0 {
                        return Err(io::Error::new(
                            io::ErrorKind::InvalidInput,
                            "--max-packets must be > 0 when provided",
                        ));
                    }
                    max_packets = Some(parsed);
                }
                "--help" | "-h" => {
                    print_help();
                    std::process::exit(0);
                }
                other => {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidInput,
                        format!("unknown argument '{other}'"),
                    ))
                }
            }
        }

        Ok(Self {
            bind_addr,
            jsonl_path,
            csv_path,
            mode,
            buffer_size,
            max_packets,
        })
    }
}

fn next_value<I>(iter: &mut I, flag: &str) -> io::Result<String>
where
    I: Iterator<Item = String>,
{
    iter.next().ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("missing value for {flag}"),
        )
    })
}

fn print_help() {
    println!(
        "udp-telemetry-receiver\n\n\
Receives UDP telemetry and writes JSONL and/or CSV logs.\n\n\
Options:\n\
  --bind ADDR           UDP bind address (default: 0.0.0.0:9000)\n\
  --jsonl PATH          JSONL output path (default: telemetry.jsonl)\n\
  --csv PATH            CSV output path (default: telemetry.csv)\n\
  --mode MODE           jsonl | csv | both (default: both)\n\
  --buffer-size BYTES   receive buffer size in bytes (default: 2048)\n\
  --max-packets COUNT   stop after COUNT packets for test runs\n\
  -h, --help            show this help message\n"
    );
}

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

fn payload_to_log_string(payload: &[u8]) -> String {
    match std::str::from_utf8(payload) {
        Ok(s) => s.replace('\n', "\\n").replace('\r', "\\r"),
        Err(_) => {
            let mut out = String::from("0x");
            for byte in payload {
                use std::fmt::Write as _;
                let _ = write!(out, "{byte:02x}");
            }
            out
        }
    }
}

fn escape_json_string(input: &str) -> String {
    let mut escaped = String::with_capacity(input.len());
    for ch in input.chars() {
        match ch {
            '\\' => escaped.push_str("\\\\"),
            '"' => escaped.push_str("\\\""),
            _ => escaped.push(ch),
        }
    }
    escaped
}

fn escape_csv_field(input: &str) -> String {
    input.replace('"', "\"\"")
}

fn main() -> io::Result<()> {
    let config = Config::from_args(env::args())?;

    let mut jsonl_writer = if config.mode.writes_jsonl() {
        Some(BufWriter::new(open_append(&config.jsonl_path)?))
    } else {
        None
    };

    if config.mode.writes_csv() {
        ensure_csv_header(&config.csv_path)?;
    }

    let mut csv_writer = if config.mode.writes_csv() {
        Some(BufWriter::new(open_append(&config.csv_path)?))
    } else {
        None
    };

    let socket = UdpSocket::bind(&config.bind_addr)?;
    eprintln!("udp-telemetry-receiver listening on {}", config.bind_addr);
    eprintln!(
        "mode={:?}, jsonl={}, csv={}, buffer_size={}, max_packets={:?}",
        config.mode, config.jsonl_path, config.csv_path, config.buffer_size, config.max_packets
    );

    let mut buf = vec![0u8; config.buffer_size];
    let mut received = 0usize;

    loop {
        let (n, src) = socket.recv_from(&mut buf)?;
        let ts = now_ms();
        let payload_utf8 = payload_to_log_string(&buf[..n]);

        if let Some(writer) = jsonl_writer.as_mut() {
            writeln!(
                writer,
                "{{\"ts_ms\":{},\"src\":\"{}\",\"len\":{},\"payload_utf8\":\"{}\"}}",
                ts,
                src,
                n,
                escape_json_string(&payload_utf8)
            )?;
            writer.flush()?;
        }

        if let Some(writer) = csv_writer.as_mut() {
            writeln!(
                writer,
                "{},{},{},{},\"{}\"",
                ts,
                src.ip(),
                src.port(),
                n,
                escape_csv_field(&payload_utf8)
            )?;
            writer.flush()?;
        }

        received += 1;
        if config.max_packets.is_some_and(|limit| received >= limit) {
            eprintln!(
                "received {} packets, exiting due to --max-packets",
                received
            );
            break;
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_defaults() {
        let cfg = Config::from_args(["udp".to_string()]).unwrap();
        assert_eq!(cfg.bind_addr, "0.0.0.0:9000");
        assert_eq!(cfg.jsonl_path, "telemetry.jsonl");
        assert_eq!(cfg.csv_path, "telemetry.csv");
        assert_eq!(cfg.mode, Mode::Both);
        assert_eq!(cfg.buffer_size, 2048);
        assert_eq!(cfg.max_packets, None);
    }

    #[test]
    fn parses_flags() {
        let cfg = Config::from_args([
            "udp".to_string(),
            "--bind".to_string(),
            "127.0.0.1:7777".to_string(),
            "--jsonl".to_string(),
            "out.jsonl".to_string(),
            "--csv".to_string(),
            "out.csv".to_string(),
            "--mode".to_string(),
            "csv".to_string(),
            "--buffer-size".to_string(),
            "4096".to_string(),
            "--max-packets".to_string(),
            "5".to_string(),
        ])
        .unwrap();

        assert_eq!(cfg.bind_addr, "127.0.0.1:7777");
        assert_eq!(cfg.jsonl_path, "out.jsonl");
        assert_eq!(cfg.csv_path, "out.csv");
        assert_eq!(cfg.mode, Mode::Csv);
        assert_eq!(cfg.buffer_size, 4096);
        assert_eq!(cfg.max_packets, Some(5));
    }

    #[test]
    fn rejects_invalid_mode() {
        let err = Config::from_args(["udp".to_string(), "--mode".to_string(), "yaml".to_string()])
            .unwrap_err();

        assert!(err.to_string().contains("invalid mode"));
    }

    #[test]
    fn renders_non_utf8_as_hex() {
        assert_eq!(
            payload_to_log_string(&[0xde, 0xad, 0xbe, 0xef]),
            "0xdeadbeef"
        );
    }

    #[test]
    fn escapes_json_and_csv() {
        let input = "hello \"robot\" \\\n";
        assert_eq!(escape_json_string(input), "hello \\\"robot\\\" \\\\\n");
        assert_eq!(escape_csv_field(input), "hello \"\"robot\"\" \\\n");
    }
}

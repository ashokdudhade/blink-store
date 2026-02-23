//! Simple text protocol for Blink-Store network interface.
//!
//! Line-based protocol (UTF-8, LF line endings):
//! - `GET <key>`         → `VALUE <base64>` or `NOT_FOUND`
//! - `SET <key> <value>` → `OK` or `ERROR <msg>` (value is rest of line)
//! - `DELETE <key>`      → `OK` or `NOT_FOUND`
//! - `USAGE`             → `USAGE <bytes>`
//! - `QUIT`              → connection close

use base64::{engine::general_purpose::STANDARD as BASE64, Engine};
use std::io::Write;

/// Response lines sent to the client.
#[derive(Debug)]
pub enum Response {
    Ok,
    Value(Vec<u8>),
    NotFound,
    Usage(u64),
    Error(String),
}

impl Response {
    pub fn write(&self, w: &mut impl Write) -> std::io::Result<()> {
        match self {
            Response::Ok => writeln!(w, "OK"),
            Response::Value(v) => writeln!(w, "VALUE {}", BASE64.encode(v)),
            Response::NotFound => writeln!(w, "NOT_FOUND"),
            Response::Usage(n) => writeln!(w, "USAGE {}", n),
            Response::Error(msg) => writeln!(w, "ERROR {}", msg.replace('\n', " ")),
        }
    }
}

/// Parses one request line; returns (command, key, value).
/// Key and value are empty when not applicable.
pub fn parse_request(line: &str) -> Option<(String, String, String)> {
    let line = line.trim();
    if line.is_empty() {
        return None;
    }
    let (cmd, rest) = line.split_once(char::is_whitespace).unwrap_or((line, ""));
    let cmd = cmd.to_uppercase();
    match cmd.as_str() {
        "GET" | "DELETE" => Some((cmd, rest.trim().to_string(), String::new())),
        "SET" => {
            let key = rest.split_whitespace().next().unwrap_or("").to_string();
            let value = rest
                .strip_prefix(key.as_str())
                .map(|s| s.trim_start().to_string())
                .unwrap_or_else(|| rest.to_string());
            Some((cmd, key, value))
        }
        "USAGE" | "QUIT" => Some((cmd, String::new(), String::new())),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_get() {
        let (cmd, key, _) = parse_request("GET foo").unwrap();
        assert_eq!(cmd, "GET");
        assert_eq!(key, "foo");
    }

    #[test]
    fn parse_set() {
        let (cmd, key, value) = parse_request("SET k v with spaces").unwrap();
        assert_eq!(cmd, "SET");
        assert_eq!(key, "k");
        assert_eq!(value, "v with spaces");
    }

    #[test]
    fn parse_set_key_only() {
        let (cmd, key, value) = parse_request("SET k").unwrap();
        assert_eq!(cmd, "SET");
        assert_eq!(key, "k");
        assert!(value.is_empty());
    }

    #[test]
    fn parse_usage_quit() {
        assert!(parse_request("USAGE").is_some());
        assert!(parse_request("QUIT").is_some());
    }
}

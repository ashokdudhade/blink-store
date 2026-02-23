//! Zero-copy text protocol for Blink-Store.
//!
//! Line-based protocol (UTF-8, LF line endings):
//! - `GET <key>`         → `VALUE <base64>` or `NOT_FOUND`
//! - `SET <key> <value>` → `OK` or `ERROR <msg>`
//! - `DELETE <key>`      → `OK` or `NOT_FOUND`
//! - `USAGE`             → `USAGE <bytes>`
//! - `QUIT`              → connection close

use base64::{engine::general_purpose::STANDARD as BASE64, Engine};
use bytes::Bytes;
use std::io::Write;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Command {
    Get,
    Set,
    Delete,
    Usage,
    Quit,
}

#[derive(Debug)]
pub enum Response {
    Ok,
    Value(Bytes),
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

/// Parses one request line into (Command, key, value) with zero heap allocations.
/// Key and value borrow from the input line.
pub fn parse_request(line: &str) -> Option<(Command, &str, &str)> {
    let line = line.trim();
    if line.is_empty() {
        return None;
    }
    let (cmd, rest) = line.split_once(char::is_whitespace).unwrap_or((line, ""));
    let rest = rest.trim_start();

    if cmd.eq_ignore_ascii_case("GET") {
        Some((Command::Get, rest.trim(), ""))
    } else if cmd.eq_ignore_ascii_case("DELETE") {
        Some((Command::Delete, rest.trim(), ""))
    } else if cmd.eq_ignore_ascii_case("SET") {
        let (key, value) = rest
            .split_once(char::is_whitespace)
            .unwrap_or((rest, ""));
        Some((Command::Set, key, value.trim_start()))
    } else if cmd.eq_ignore_ascii_case("USAGE") {
        Some((Command::Usage, "", ""))
    } else if cmd.eq_ignore_ascii_case("QUIT") {
        Some((Command::Quit, "", ""))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_get() {
        let (cmd, key, _) = parse_request("GET foo").unwrap();
        assert_eq!(cmd, Command::Get);
        assert_eq!(key, "foo");
    }

    #[test]
    fn parse_set() {
        let (cmd, key, value) = parse_request("SET k v with spaces").unwrap();
        assert_eq!(cmd, Command::Set);
        assert_eq!(key, "k");
        assert_eq!(value, "v with spaces");
    }

    #[test]
    fn parse_set_key_only() {
        let (cmd, key, value) = parse_request("SET k").unwrap();
        assert_eq!(cmd, Command::Set);
        assert_eq!(key, "k");
        assert!(value.is_empty());
    }

    #[test]
    fn parse_usage_quit() {
        let (cmd, _, _) = parse_request("USAGE").unwrap();
        assert_eq!(cmd, Command::Usage);
        let (cmd, _, _) = parse_request("QUIT").unwrap();
        assert_eq!(cmd, Command::Quit);
    }

    #[test]
    fn parse_case_insensitive() {
        let (cmd, key, _) = parse_request("get FOO").unwrap();
        assert_eq!(cmd, Command::Get);
        assert_eq!(key, "FOO");
    }
}

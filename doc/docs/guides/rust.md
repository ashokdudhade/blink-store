---
title: Rust
---

# Rust

Connect to Blink-Store from Rust using the standard library plus the `base64` crate.

---

## Prerequisites

- Rust 1.70+ (install from [rustup.rs](https://rustup.rs))
- `base64 = "0.22"` in `Cargo.toml`
- Blink-Store server running ([Installation](../installation))

---

## Interactive client

A complete REPL client. Add `base64 = "0.22"` to your `[dependencies]`, then save as `src/main.rs`.

```rust
use base64::Engine;
use std::io::{self, BufRead, BufReader, Write};
use std::net::TcpStream;

fn main() -> io::Result<()> {
    let addr = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:8765".to_string());

    let mut stream = TcpStream::connect(&addr)?;
    let mut reader = BufReader::new(stream.try_clone()?);

    eprintln!("Connected to {addr}");
    eprintln!("Commands: SET <key> <value> | GET <key> | DELETE <key> | USAGE | QUIT\n");

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let cmd = line?.trim().to_string();
        if cmd.is_empty() {
            continue;
        }
        if cmd.eq_ignore_ascii_case("QUIT") {
            break;
        }

        writeln!(stream, "{cmd}")?;
        stream.flush()?;

        let mut resp = String::new();
        reader.read_line(&mut resp)?;
        let resp = resp.trim();

        if let Some(b64) = resp.strip_prefix("VALUE ") {
            match base64::engine::general_purpose::STANDARD.decode(b64.trim()) {
                Ok(bytes) => println!("{}", String::from_utf8_lossy(&bytes)),
                Err(_) => println!("{resp}"),
            }
        } else {
            println!("{resp}");
        }
    }
    Ok(())
}
```

**Run:**

```bash
cargo run
```

```text
Connected to 127.0.0.1:8765
Commands: SET <key> <value> | GET <key> | DELETE <key> | USAGE | QUIT

SET framework rust
OK
GET framework
rust
QUIT
```

---

## One-off commands

A reusable helper function:

```rust
use base64::Engine;
use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;

fn blink(command: &str, addr: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut stream = TcpStream::connect(addr)?;
    let mut reader = BufReader::new(stream.try_clone()?);

    writeln!(stream, "{command}")?;
    stream.flush()?;

    let mut resp = String::new();
    reader.read_line(&mut resp)?;
    let resp = resp.trim();

    if let Some(b64) = resp.strip_prefix("VALUE ") {
        let bytes = base64::engine::general_purpose::STANDARD.decode(b64.trim())?;
        Ok(String::from_utf8(bytes)?)
    } else {
        Ok(resp.to_string())
    }
}

fn main() {
    let addr = "127.0.0.1:8765";
    blink("SET version 1.0", addr).unwrap();
    let val = blink("GET version", addr).unwrap();
    println!("{val}"); // → 1.0
}
```

---

## Async client (Tokio)

For async applications, use `tokio::net::TcpStream`:

```rust
use base64::Engine;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;

async fn blink(command: &str, addr: &str) -> Result<String, Box<dyn std::error::Error>> {
    let stream = TcpStream::connect(addr).await?;
    let (reader, mut writer) = stream.into_split();
    let mut reader = BufReader::new(reader);

    writer.write_all(format!("{command}\n").as_bytes()).await?;
    writer.flush().await?;

    let mut resp = String::new();
    reader.read_line(&mut resp).await?;
    let resp = resp.trim();

    if let Some(b64) = resp.strip_prefix("VALUE ") {
        let bytes = base64::engine::general_purpose::STANDARD.decode(b64.trim())?;
        Ok(String::from_utf8(bytes)?)
    } else {
        Ok(resp.to_string())
    }
}
```

Add to `Cargo.toml`: `tokio = { version = "1", features = ["full"] }` and `base64 = "0.22"`.

---

## Key concepts

| Concept | Rust API |
|---------|---------|
| TCP connection | `TcpStream::connect(addr)` |
| Line reading | `BufReader::new(stream).read_line()` |
| Base64 decode | `base64::engine::general_purpose::STANDARD.decode()` |
| Error handling | `Result<T, E>` — no panics |

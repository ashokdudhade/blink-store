//! Example client for Blink-Store TCP/Unix protocol.
//!
//! Usage:
//!   cargo run --example blink_client -- --tcp 127.0.0.1:8765
//!   cargo run --example blink_client -- --unix /tmp/blink-store.sock  (Unix only)

use base64::{engine::general_purpose::STANDARD as BASE64, Engine};
use clap::Parser;
use std::io::{self, BufRead, Write};
use std::path::PathBuf;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt};

#[derive(Parser)]
struct Args {
    #[arg(long)]
    tcp: Option<String>,
    #[arg(long)]
    unix: Option<PathBuf>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    if args.tcp.is_none() && args.unix.is_none() {
        eprintln!("Use --tcp <addr> or --unix <path>");
        std::process::exit(1);
    }
    if args.tcp.is_some() && args.unix.is_some() {
        eprintln!("Use only one of --tcp or --unix");
        std::process::exit(1);
    }

    if let Some(addr) = &args.tcp {
        run_tcp(addr).await?;
    } else if let Some(path) = &args.unix {
        run_unix(path).await?;
    }
    Ok(())
}

async fn run_tcp(addr: &str) -> anyhow::Result<()> {
    let stream = tokio::net::TcpStream::connect(addr).await?;
    let (r, mut w) = stream.into_split();
    let mut reader = tokio::io::BufReader::new(r);
    repl_loop(&mut reader, &mut w).await
}

#[cfg(unix)]
async fn run_unix(path: &std::path::Path) -> anyhow::Result<()> {
    let stream = tokio::net::UnixStream::connect(path).await?;
    let (r, mut w) = stream.into_split();
    let mut reader = tokio::io::BufReader::new(r);
    repl_loop(&mut reader, &mut w).await
}

#[cfg(not(unix))]
async fn run_unix(_path: &std::path::Path) -> anyhow::Result<()> {
    anyhow::bail!("--unix not supported on this platform");
}

async fn repl_loop<R, W>(reader: &mut R, writer: &mut W) -> anyhow::Result<()>
where
    R: AsyncBufReadExt + Unpin,
    W: AsyncWriteExt + Unpin,
{
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    writeln!(stdout, "Commands: GET <key> | SET <key> <value> | DELETE <key> | USAGE | QUIT")?;
    stdout.flush()?;

    for line in stdin.lock().lines() {
        let line = line?;
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let cmd_upper = line.split_whitespace().next().unwrap_or("").to_uppercase();
        if cmd_upper == "QUIT" {
            break;
        }

        let mut req = line.to_string();
        req.push('\n');
        writer.write_all(req.as_bytes()).await?;
        writer.flush().await?;

        let mut resp = String::new();
        reader.read_line(&mut resp).await?;
        let resp = resp.trim();

        if resp.starts_with("VALUE ") {
            let b64 = resp.strip_prefix("VALUE ").unwrap_or("");
            match BASE64.decode(b64.trim()) {
                Ok(bytes) => writeln!(stdout, "{}", String::from_utf8_lossy(&bytes))?,
                Err(_) => writeln!(stdout, "{}", resp)?,
            }
        } else {
            writeln!(stdout, "{}", resp)?;
        }
        stdout.flush()?;
    }

    Ok(())
}

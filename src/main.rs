//! CLI for Blink Store.

use anyhow::{Context, Result};
use blink_store::MemoryEngine;
use bytes::Bytes;
use clap::Parser;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;
use tracing::info;

#[derive(Parser)]
#[command(name = "blink-store")]
#[command(about = "In-memory key-value store with LRU eviction")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(clap::Subcommand)]
enum Command {
    /// Run the store (REPL) with optional log retention
    Run {
        /// Memory limit in bytes.
        #[arg(long, default_value = "10485760")]
        memory_limit: u64,

        /// Prune log files older than this many minutes (0 = disable).
        #[arg(long, default_value = "60")]
        retention_minutes: u64,

        /// Directory for log files (if unset, logs only to stdout).
        #[arg(long)]
        log_dir: Option<PathBuf>,
    },

    /// Serve storage over TCP and/or Unix socket
    Serve {
        /// Memory limit in bytes.
        #[arg(long, default_value = "10485760")]
        memory_limit: u64,

        /// TCP listen address (e.g. 0.0.0.0:8765).
        #[arg(long)]
        tcp: Option<String>,

        /// Unix socket path (e.g. /tmp/blink-store.sock). Unix only.
        #[arg(long)]
        unix: Option<PathBuf>,

        /// Prune log files older than this many minutes (0 = disable).
        #[arg(long, default_value = "60")]
        retention_minutes: u64,

        /// Directory for log files.
        #[arg(long)]
        log_dir: Option<PathBuf>,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::Run {
            memory_limit,
            retention_minutes,
            log_dir,
        } => {
            let _guard = blink_store::logging::init_tracing(
                log_dir.as_deref(),
                Some("blink_store=info,info"),
            )?;

            if let Some(ref dir) = log_dir {
                if retention_minutes > 0 {
                    let dir = dir.clone();
                    let retention = Duration::from_secs(retention_minutes * 60);
                    blink_store::logging::spawn_log_retention_worker(dir, retention);
                }
            }

            let store = Arc::new(MemoryEngine::new(memory_limit)?);
            info!(action = "start", memory_limit, "Store ready");
            run_repl(store).await?;
        }

        Command::Serve {
            memory_limit,
            tcp,
            unix,
            retention_minutes,
            log_dir,
        } => {
            if tcp.is_none() && unix.is_none() {
                anyhow::bail!("At least one of --tcp or --unix is required");
            }
            let _guard = blink_store::logging::init_tracing(
                log_dir.as_deref(),
                Some("blink_store=info,info"),
            )?;

            if let Some(ref dir) = log_dir {
                if retention_minutes > 0 {
                    let dir = dir.clone();
                    let retention = Duration::from_secs(retention_minutes * 60);
                    blink_store::logging::spawn_log_retention_worker(dir, retention);
                }
            }

            let store = Arc::new(MemoryEngine::new(memory_limit)?);

            if let Some(ref addr) = tcp {
                let store_tcp = store.clone();
                let addr = addr.clone();
                tokio::spawn(async move {
                    let _ = blink_store::run_tcp(&addr, store_tcp).await;
                });
            }
            #[cfg(unix)]
            if let Some(ref path) = unix {
                let store_unix = store.clone();
                let path = path.clone();
                tokio::spawn(async move {
                    let _ = blink_store::run_unix(&path, store_unix).await;
                });
            }
            #[cfg(not(unix))]
            if unix.is_some() {
                anyhow::bail!("--unix is only supported on Unix platforms");
            }

            info!(action = "serve", memory_limit, "Server ready");
            tokio::signal::ctrl_c().await.context("wait for ctrl_c")?;
        }
    }

    Ok(())
}

/// Simple REPL: get <k>, set <k> <v>, delete <k>, usage, quit.
async fn run_repl(store: std::sync::Arc<MemoryEngine>) -> Result<()> {
    use blink_store::BlinkStorage;
    use std::io::{self, BufRead, Write};

    let stdin = io::stdin();
    let mut stdout = io::stdout();

    writeln!(stdout, "Commands: get <key> | set <key> <value> | delete <key> | usage | quit")?;
    stdout.flush()?;

    for line in stdin.lock().lines() {
        let line = line?;
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let (cmd, rest) = line
            .split_once(char::is_whitespace)
            .map(|(c, r)| (c, r.trim()))
            .unwrap_or((line, ""));

        match cmd {
            "quit" | "exit" => break,
            "usage" => {
                let n = store.current_usage_bytes()?;
                writeln!(stdout, "usage: {} bytes", n)?;
            }
            "get" => {
                let key = rest;
                if key.is_empty() {
                    writeln!(stdout, "get <key>")?;
                } else {
                    match store.get(key) {
                        Ok(Some(v)) => writeln!(stdout, "{}", String::from_utf8_lossy(&v))?,
                        Ok(None) => writeln!(stdout, "(not found)")?,
                        Err(e) => writeln!(stdout, "error: {}", e)?,
                    }
                }
            }
            "set" => {
                let key = rest.split_whitespace().next().unwrap_or("");
                let value = rest
                    .strip_prefix(key)
                    .map(|s| s.trim_start())
                    .unwrap_or("");
                if key.is_empty() {
                    writeln!(stdout, "set <key> <value>")?;
                } else {
                    match store.set(key, Bytes::copy_from_slice(value.as_bytes())) {
                        Ok(()) => writeln!(stdout, "ok")?,
                        Err(e) => writeln!(stdout, "error: {}", e)?,
                    }
                }
            }
            "delete" => {
                let key = rest;
                if key.is_empty() {
                    writeln!(stdout, "delete <key>")?;
                } else {
                    match store.delete(key) {
                        Ok(true) => writeln!(stdout, "deleted")?,
                        Ok(false) => writeln!(stdout, "(not found)")?,
                        Err(e) => writeln!(stdout, "error: {}", e)?,
                    }
                }
            }
            _ => writeln!(stdout, "Unknown command. Use: get | set | delete | usage | quit")?,
        }
        stdout.flush()?;
    }

    Ok(())
}

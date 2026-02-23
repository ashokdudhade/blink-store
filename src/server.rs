//! TCP and Unix socket server exposing MemoryEngine over the text protocol.

use crate::engine::{BlinkStorage, MemoryEngine};
use crate::error::BlinkError;
use crate::protocol::{parse_request, Command, Response};
use bytes::Bytes;
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader as AsyncBufReader};
use tokio::net::{TcpListener, TcpStream};
use tracing::info;

pub async fn run_tcp(addr: &str, store: Arc<MemoryEngine>) -> Result<(), BlinkError> {
    let listener = TcpListener::bind(addr)
        .await
        .map_err(|e| BlinkError::Internal(format!("TCP bind {}: {}", addr, e)))?;
    info!(action = "tcp_listen", addr = %addr);
    loop {
        let (stream, peer) = listener
            .accept()
            .await
            .map_err(|e| BlinkError::Internal(format!("accept: {}", e)))?;
        let store = store.clone();
        tokio::spawn(async move {
            if let Err(e) = serve_stream(stream, store).await {
                info!(peer = ?peer, error = %e, action = "serve_stream_error");
            }
        });
    }
}

#[cfg(unix)]
pub async fn run_unix(
    path: &std::path::Path,
    store: Arc<MemoryEngine>,
) -> Result<(), BlinkError> {
    use tokio::net::UnixListener;
    let _ = std::fs::remove_file(path);
    let listener = UnixListener::bind(path)
        .map_err(|e| BlinkError::Internal(format!("Unix bind {:?}: {}", path, e)))?;
    info!(action = "unix_listen", path = ?path);
    loop {
        let (stream, _) = listener
            .accept()
            .await
            .map_err(|e| BlinkError::Internal(format!("unix accept: {}", e)))?;
        let store = store.clone();
        tokio::spawn(async move {
            let _ = serve_unix_stream(stream, store).await;
        });
    }
}

async fn serve_stream(
    mut stream: TcpStream,
    store: Arc<MemoryEngine>,
) -> Result<(), BlinkError> {
    let (reader, mut writer) = stream.split();
    let mut reader = AsyncBufReader::new(reader);
    let mut line = String::new();
    let mut resp_buf = Vec::with_capacity(256);

    loop {
        line.clear();
        let n = reader
            .read_line(&mut line)
            .await
            .map_err(|e| BlinkError::Internal(format!("read: {}", e)))?;
        if n == 0 {
            break;
        }

        let response = match parse_request(line.trim()) {
            Some((Command::Quit, _, _)) => break,
            Some((cmd, key, value)) => handle_command(cmd, key, value, &store),
            None => Response::Error("unknown command".into()),
        };

        resp_buf.clear();
        response
            .write(&mut resp_buf)
            .map_err(|e| BlinkError::Internal(e.to_string()))?;
        writer
            .write_all(&resp_buf)
            .await
            .map_err(|e| BlinkError::Internal(format!("write: {}", e)))?;
    }
    Ok(())
}

#[cfg(unix)]
async fn serve_unix_stream(
    stream: tokio::net::UnixStream,
    store: Arc<MemoryEngine>,
) -> Result<(), BlinkError> {
    let (reader, mut writer) = stream.into_split();
    let mut reader = AsyncBufReader::new(reader);
    let mut line = String::new();
    let mut resp_buf = Vec::with_capacity(256);

    loop {
        line.clear();
        let n = reader
            .read_line(&mut line)
            .await
            .map_err(|e| BlinkError::Internal(format!("read: {}", e)))?;
        if n == 0 {
            break;
        }

        let response = match parse_request(line.trim()) {
            Some((Command::Quit, _, _)) => break,
            Some((cmd, key, value)) => handle_command(cmd, key, value, &store),
            None => Response::Error("unknown command".into()),
        };

        resp_buf.clear();
        response
            .write(&mut resp_buf)
            .map_err(|e| BlinkError::Internal(e.to_string()))?;
        writer
            .write_all(&resp_buf)
            .await
            .map_err(|e| BlinkError::Internal(format!("write: {}", e)))?;
    }
    Ok(())
}

fn handle_command(cmd: Command, key: &str, value: &str, store: &MemoryEngine) -> Response {
    match cmd {
        Command::Get => {
            if key.is_empty() {
                return Response::Error("GET requires key".into());
            }
            match store.get(key) {
                Ok(Some(v)) => Response::Value(v),
                Ok(None) => Response::NotFound,
                Err(e) => Response::Error(e.to_string()),
            }
        }
        Command::Set => {
            if key.is_empty() {
                return Response::Error("SET requires key".into());
            }
            match store.set(key, Bytes::copy_from_slice(value.as_bytes())) {
                Ok(()) => Response::Ok,
                Err(e) => Response::Error(e.to_string()),
            }
        }
        Command::Delete => {
            if key.is_empty() {
                return Response::Error("DELETE requires key".into());
            }
            match store.delete(key) {
                Ok(true) => Response::Ok,
                Ok(false) => Response::NotFound,
                Err(e) => Response::Error(e.to_string()),
            }
        }
        Command::Usage => match store.current_usage_bytes() {
            Ok(n) => Response::Usage(n),
            Err(e) => Response::Error(e.to_string()),
        },
        Command::Quit => unreachable!(),
    }
}

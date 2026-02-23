//! Lightweight TCP and Unix socket server exposing BlinkStorage over the text protocol.

use crate::error::BlinkError;
use crate::protocol::{parse_request, Response};
use crate::BlinkStorage;
use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader as AsyncBufReader};
use tokio::net::{TcpListener, TcpStream};
use tracing::{info, instrument};

/// Runs a TCP listener; accepts connections and serves each with the text protocol.
pub async fn run_tcp(addr: &str, store: Arc<dyn BlinkStorage>) -> Result<(), BlinkError> {
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

/// Runs a Unix domain socket listener (Unix only).
#[cfg(unix)]
pub async fn run_unix(path: &std::path::Path, store: Arc<dyn BlinkStorage>) -> Result<(), BlinkError> {
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

#[instrument(skip(stream, store))]
async fn serve_stream(
    mut stream: TcpStream,
    store: Arc<dyn BlinkStorage>,
) -> Result<(), BlinkError> {
    let (reader, mut writer) = stream.split();
    let mut reader = AsyncBufReader::new(reader);
    let mut line = String::new();
    loop {
        line.clear();
        let n = reader
            .read_line(&mut line)
            .await
            .map_err(|e| BlinkError::Internal(format!("read: {}", e)))?;
        if n == 0 {
            break;
        }
        let response = handle_request(line.trim(), &*store).await;
        if let Response::Error(ref msg) = response {
            if msg == "QUIT" {
                break;
            }
        }
        let mut buf = Vec::with_capacity(256);
        response.write(&mut buf).map_err(|e| BlinkError::Internal(e.to_string()))?;
        writer
            .write_all(&buf)
            .await
            .map_err(|e| BlinkError::Internal(format!("write: {}", e)))?;
    }
    Ok(())
}

#[cfg(unix)]
#[instrument(skip(stream, store))]
async fn serve_unix_stream(
    stream: tokio::net::UnixStream,
    store: Arc<dyn BlinkStorage>,
) -> Result<(), BlinkError> {
    let (reader, mut writer) = stream.into_split();
    let mut reader = AsyncBufReader::new(reader);
    let mut line = String::new();
    loop {
        line.clear();
        let n = reader
            .read_line(&mut line)
            .await
            .map_err(|e| BlinkError::Internal(format!("read: {}", e)))?;
        if n == 0 {
            break;
        }
        let response = handle_request(line.trim(), &*store).await;
        if let Response::Error(ref msg) = response {
            if msg == "QUIT" {
                break;
            }
        }
        let mut buf = Vec::with_capacity(256);
        response.write(&mut buf).map_err(|e| BlinkError::Internal(e.to_string()))?;
        writer
            .write_all(&buf)
            .await
            .map_err(|e| BlinkError::Internal(format!("write: {}", e)))?;
    }
    Ok(())
}

async fn handle_request(line: &str, store: &dyn BlinkStorage) -> Response {
    let Some((cmd, key, value)) = parse_request(line) else {
        return Response::Error("unknown command".into());
    };
    match cmd.as_str() {
        "GET" => {
            if key.is_empty() {
                return Response::Error("GET requires key".into());
            }
            match store.get(&key).await {
                Ok(Some(v)) => Response::Value(v),
                Ok(None) => Response::NotFound,
                Err(e) => Response::Error(e.to_string()),
            }
        }
        "SET" => {
            if key.is_empty() {
                return Response::Error("SET requires key".into());
            }
            match store.set(&key, value.as_bytes().to_vec()).await {
                Ok(()) => Response::Ok,
                Err(e) => Response::Error(e.to_string()),
            }
        }
        "DELETE" => {
            if key.is_empty() {
                return Response::Error("DELETE requires key".into());
            }
            match store.delete(&key).await {
                Ok(true) => Response::Ok,
                Ok(false) => Response::NotFound,
                Err(e) => Response::Error(e.to_string()),
            }
        }
        "USAGE" => match store.current_usage_bytes().await {
            Ok(n) => Response::Usage(n),
            Err(e) => Response::Error(e.to_string()),
        },
        "QUIT" => return Response::Error("QUIT".into()),
        _ => Response::Error("unknown command".into()),
    }
}

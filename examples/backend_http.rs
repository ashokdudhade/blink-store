//! Minimal HTTP backend that uses Blink Store (TCP) as cache.
//! GET /<key> returns cached value; POST /<key> with body sets value.
//!
//! Start blink-store: cargo run -- serve --tcp 127.0.0.1:8765
//! Run this:          cargo run --example backend_http -- --store 127.0.0.1:8765 --port 8080

use base64::{engine::general_purpose::STANDARD as BASE64, Engine};
use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

/// One TCP connection per request to Blink Store.
async fn store_request(addr: &str, cmd: &str, key: &str, value: &[u8]) -> std::io::Result<String> {
    let mut stream = TcpStream::connect(addr).await?;
    let (reader, mut writer) = stream.split();

    let req = if value.is_empty() {
        format!("{} {}\n", cmd, key)
    } else {
        let v = String::from_utf8_lossy(value);
        format!("{} {} {}\n", cmd, key, v)
    };
    writer.write_all(req.as_bytes()).await?;
    writer.flush().await?;

    let mut line = String::new();
    let mut buf_reader = tokio::io::BufReader::new(reader);
    buf_reader.read_line(&mut line).await?;
    Ok(line)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let args: Vec<String> = std::env::args().collect();
    let store_addr = args
        .iter()
        .position(|a| a == "--store")
        .and_then(|i| args.get(i + 1))
        .cloned()
        .unwrap_or_else(|| "127.0.0.1:8765".to_string());
    let port: u16 = args
        .iter()
        .position(|a| a == "--port")
        .and_then(|i| args.get(i + 1))
        .and_then(|s| s.parse().ok())
        .unwrap_or(8080);

    let listen = format!("0.0.0.0:{}", port);
    let listener = TcpListener::bind(&listen).await?;
    eprintln!("HTTP backend on {} (store at {})", listen, store_addr);

    loop {
        let (stream, _) = listener.accept().await?;
        let store_addr = store_addr.clone();
        tokio::spawn(async move {
            let _ = handle_http(stream, &store_addr).await;
        });
    }
}

async fn handle_http(
    mut stream: TcpStream,
    store_addr: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let (reader, mut writer) = stream.split();
    let mut buf_reader = tokio::io::BufReader::new(reader);
    let mut first_line = String::new();
    buf_reader.read_line(&mut first_line).await?;
    let first_line = first_line.trim_end();
    let parts: Vec<&str> = first_line.split_whitespace().collect();
    let (method, path) = if parts.len() >= 2 {
        (parts[0], parts[1])
    } else {
        send_response(&mut writer, 400, b"Bad Request").await?;
        return Ok(());
    };
    let key = path.trim_start_matches('/');
    if key.is_empty() {
        send_response(&mut writer, 400, b"Bad Request: use /<key>").await?;
        return Ok(());
    }

    if method == "GET" {
        let resp = store_request(store_addr, "GET", key, &[]).await?;
        let resp = resp.trim_end();
        if resp == "NOT_FOUND" {
            send_response(&mut writer, 404, b"Not Found").await?;
            return Ok(());
        }
        if let Some(stripped) = resp.strip_prefix("VALUE ") {
            match BASE64.decode(stripped.trim().as_bytes()) {
                Ok(v) => send_response(&mut writer, 200, &v).await?,
                Err(_) => send_response(&mut writer, 502, b"Bad Gateway").await?,
            }
        } else {
            send_response(&mut writer, 502, b"Bad Gateway").await?;
        }
    } else if method == "POST" {
        let mut content_length: Option<usize> = None;
        loop {
            let mut line = String::new();
            buf_reader.read_line(&mut line).await?;
            let line = line.trim_end();
            if line.is_empty() {
                break;
            }
            if line.to_lowercase().starts_with("content-length:") {
                content_length = line.split(':').nth(1).and_then(|s| s.trim().parse().ok());
            }
        }
        let mut body = Vec::new();
        if let Some(len) = content_length {
            body.resize(len, 0);
            let mut n = 0;
            while n < len {
                n += buf_reader.read(&mut body[n..]).await?;
            }
        } else {
            let mut buf = [0u8; 4096];
            loop {
                let n = buf_reader.read(&mut buf).await?;
                if n == 0 {
                    break;
                }
                body.extend_from_slice(&buf[..n]);
            }
        }
        let resp = store_request(store_addr, "SET", key, &body).await?;
        if resp.trim_end().starts_with("OK") {
            send_response(&mut writer, 204, b"").await?;
        } else {
            send_response(&mut writer, 502, b"Bad Gateway").await?;
        }
    } else {
        send_response(&mut writer, 405, b"Method Not Allowed").await?;
    }
    Ok(())
}

async fn send_response<W>(w: &mut W, status: u16, body: &[u8]) -> std::io::Result<()>
where
    W: AsyncWriteExt + Unpin,
{
    let status_line = match status {
        200 => "HTTP/1.0 200 OK",
        204 => "HTTP/1.0 204 No Content",
        400 => "HTTP/1.0 400 Bad Request",
        404 => "HTTP/1.0 404 Not Found",
        405 => "HTTP/1.0 405 Method Not Allowed",
        502 => "HTTP/1.0 502 Bad Gateway",
        _ => "HTTP/1.0 500 Internal Server Error",
    };
    let content_len = body.len();
    let h = if content_len > 0 {
        format!("{}\r\nContent-Length: {}\r\n\r\n", status_line, content_len)
    } else {
        format!("{}\r\n\r\n", status_line)
    };
    w.write_all(h.as_bytes()).await?;
    if content_len > 0 {
        w.write_all(body).await?;
    }
    w.flush().await?;
    Ok(())
}

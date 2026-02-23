# Language-Specific Integration Guides

Connect to Blink Store over TCP (or Unix socket on Unix) using the line-based text protocol. See [Protocol Specification](PROTOCOL_SPEC.md).

## Node.js

Use `net.Socket`; send one line per request, read one line per response. Buffer handling: accumulate data until `\n`, then parse.

```javascript
const net = require('net');
const sock = net.createConnection(port, host, () => {});
let buf = '';
sock.on('data', (c) => { buf += c; /* split on \n, parse VALUE base64 */ });
sock.write('GET mykey\n');
```

Full example: `examples/clients/node/blink_client.js`, `examples/clients/node/backend_app.js`.

## Python

Use `socket` with context managers; `makefile('r')` for line reading.

```python
import socket
with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
    s.connect((host, port))
    r = s.makefile('r', encoding='utf-8', newline='\n')
    s.sendall(b'GET mykey\n')
    line = r.readline()
```

Full example: `examples/clients/python/blink_client.py`, `examples/clients/python/backend_app.py`.

## Rust

Use `tokio::net::TcpStream` (or `UnixStream`); `AsyncBufReadExt::read_line` for line-based I/O.

```rust
use tokio::io::{AsyncBufReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
let mut stream = TcpStream::connect(addr).await?;
stream.write_all(b"GET mykey\n").await?;
let mut line = String::new();
BufReader::new(stream).read_line(&mut line).await?;
```

Full example: `examples/blink_client.rs`, `examples/backend_http.rs`.

## Go

Use `net.Dial("tcp", addr)` and `bufio.Scanner` or `bufio.ReadString('\n')`.

```go
conn, _ := net.Dial("tcp", addr)
fmt.Fprintf(conn, "GET mykey\n")
line, _ := bufio.NewReader(conn).ReadString('\n')
```

Full example: `examples/clients/go/blink_client.go`, `examples/clients/go/backend_app.go`.

## Java / C#

- **Java**: Use `java.net.Socket`, `BufferedReader.readLine()` and `OutputStream.write()`; decode VALUE responses with `Base64.getDecoder()`.
- **C#**: Use `TcpClient`, `StreamReader.ReadLine()` and `StreamWriter.WriteLine()`; decode with `Convert.FromBase64String()`.

Connection pooling: reuse a small pool of sockets per process; avoid one connection per request under load.

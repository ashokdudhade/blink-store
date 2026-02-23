# Language guides

Blink-Store is protocol-first: any language that can open a TCP (or Unix) socket and send/receive lines can talk to it. This page lists the official examples and how to run them.

## Prerequisites

1. Build a local distribution (or use `cargo run` for Rust examples):
   ```bash
   ./scripts/build-dist.sh
   ```
2. Start the server:
   ```bash
   ./dist/blink-store serve --tcp 127.0.0.1 8765
   ```

## REPL clients

Interactive clients that send one command per line (GET, SET, DELETE, USAGE, QUIT).

| Language | Command |
|----------|---------|
| **Rust** | `./dist/blink_client --tcp 127.0.0.1:8765` |
| **Python** | `python examples/clients/python/blink_client.py` |
| **Node.js** | `node examples/clients/node/blink_client.js` |
| **Go** | `go run examples/clients/go/blink_client.go` |
| **Shell** | `bash examples/clients/shell/blink_client.sh` |

Default host is `127.0.0.1` and port `8765`; Python, Node, and Go accept optional `[host [port]]` arguments.

## HTTP backends

Minimal HTTP servers that use Blink-Store as a cache: `GET /<key>` returns the value; `POST /<key>` with body sets it. Set `BLINK_STORE=host:port` (default `127.0.0.1:8765`) and optionally `PORT` (default `8080`).

| Language | Command |
|----------|---------|
| **Rust** | `./dist/backend_http --store 127.0.0.1:8765 --port 8080` |
| **Python** | `BLINK_STORE=127.0.0.1:8765 python examples/clients/python/backend_app.py` |
| **Node.js** | `BLINK_STORE=127.0.0.1:8765 node examples/clients/node/backend_app.js` |
| **Go** | `BLINK_STORE=127.0.0.1:8765 go run examples/clients/go/backend_app.go` |

Example with `curl`:

```bash
curl -X POST http://localhost:8080/foo -d 'hello'
curl http://localhost:8080/foo
# → hello
```

## Integration notes

- **Node.js** — Use `net.Socket`; buffer incoming data and split on `\n`; decode `VALUE <base64>` with `Buffer.from(..., 'base64')`.
- **Python** — Use `socket.socket` and `makefile('r')` for line reading; decode base64 with `base64.b64decode`.
- **Rust** — Use `tokio::net::TcpStream` and `AsyncBufReadExt::read_line` for async line I/O.
- **Go** — Use `net.Dial("tcp", addr)` and `bufio.ReadString('\n')`.

For more detail and connection-pooling tips (e.g. Java/C#), see the repo: `docs/INTEGRATION_GUIDES.md`.

# Blink-Store example clients

Same line-based protocol: `GET <key>`, `SET <key> <value>`, `DELETE <key>`, `USAGE`, `QUIT`.  
Responses: `OK`, `VALUE <base64>`, `NOT_FOUND`, `USAGE <n>`, `ERROR <msg>`.

Start the store first: `cargo run -- serve --tcp 127.0.0.1 8765`

## REPL clients (default 127.0.0.1:8765)

| Language | Run |
|----------|-----|
| **Rust** | `cargo run --example blink_client -- --tcp 127.0.0.1:8765` |
| **Python** | `python examples/clients/python/blink_client.py [host [port]]` |
| **Node.js** | `node examples/clients/node/blink_client.js [host [port]]` |
| **Go** | `go run examples/clients/go/blink_client.go [host [port]]` |
| **Shell** | `bash examples/clients/shell/blink_client.sh [host [port]]` |

## Backend usage (HTTP API using Blink-Store as cache)

Each runs an HTTP server: `GET /<key>` returns value, `POST /<key>` with body sets value.  
Set `BLINK_STORE=host:port` (default 127.0.0.1:8765) and optionally `PORT` (default 8080).

| Language | Run |
|----------|-----|
| **Rust** | `cargo run --example backend_http -- --store 127.0.0.1:8765 --port 8080` |
| **Python** | `BLINK_STORE=127.0.0.1:8765 python examples/clients/python/backend_app.py` |
| **Node.js** | `BLINK_STORE=127.0.0.1:8765 node examples/clients/node/backend_app.js` |
| **Go** | `BLINK_STORE=127.0.0.1:8765 go run examples/clients/go/backend_app.go` |

Example: `curl -X POST http://localhost:8080/foo -d 'hello'` then `curl http://localhost:8080/foo` → `hello`.

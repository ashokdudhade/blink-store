# Blink-Store example clients

Same line-based protocol: `GET <key>`, `SET <key> <value>`, `DELETE <key>`, `USAGE`, `QUIT`.  
Responses: `OK`, `VALUE <base64>`, `NOT_FOUND`, `USAGE <n>`, `ERROR <msg>`.

## Server (no clone)

Install the **latest** server with curl and start it:

```bash
curl -sSLf https://raw.githubusercontent.com/ashokdudhade/blink-store/main/scripts/install-from-github.sh | bash -s -- latest ./bin
./bin/blink-store serve --tcp 127.0.0.1 8765
```

## REPL clients (default 127.0.0.1:8765)

Run **without cloning** by downloading the script:

| Language | Command (no clone) |
|----------|--------------------|
| **Python** | `curl -sSLf https://raw.githubusercontent.com/ashokdudhade/blink-store/main/examples/clients/python/blink_client.py -o blink_client.py && python3 blink_client.py 127.0.0.1 8765` |
| **Node.js** | `curl -sSLf https://raw.githubusercontent.com/ashokdudhade/blink-store/main/examples/clients/node/blink_client.js -o blink_client.js && node blink_client.js 127.0.0.1 8765` |
| **Go** | `curl -sSLf https://raw.githubusercontent.com/ashokdudhade/blink-store/main/examples/clients/go/blink_client.go -o blink_client.go && go run blink_client.go 127.0.0.1 8765` |
| **Shell** | `curl -sSLf https://raw.githubusercontent.com/ashokdudhade/blink-store/main/examples/clients/shell/blink_client.sh -o blink_client.sh && bash blink_client.sh 127.0.0.1 8765` |

With the repo: `python examples/clients/python/blink_client.py`, etc. Rust: `cargo run --example blink_client -- --tcp 127.0.0.1:8765`.

## Backend usage (HTTP API using Blink-Store as cache)

Each runs an HTTP server: `GET /<key>` returns value, `POST /<key>` with body sets value.  
Set `BLINK_STORE=host:port` (default 127.0.0.1:8765) and optionally `PORT` (default 8080).

| Language | Command (no clone) |
|----------|--------------------|
| **Python** | `curl -sSLf https://raw.githubusercontent.com/ashokdudhade/blink-store/main/examples/clients/python/backend_app.py -o backend_app.py && BLINK_STORE=127.0.0.1:8765 python3 backend_app.py` |
| **Node.js** | `curl -sSLf https://raw.githubusercontent.com/ashokdudhade/blink-store/main/examples/clients/node/backend_app.js -o backend_app.js && BLINK_STORE=127.0.0.1:8765 node backend_app.js` |
| **Go** | `curl -sSLf https://raw.githubusercontent.com/ashokdudhade/blink-store/main/examples/clients/go/backend_app.go -o backend_app.go && BLINK_STORE=127.0.0.1:8765 go run backend_app.go` |

Example: `curl -X POST http://localhost:8080/foo -d 'hello'` then `curl http://localhost:8080/foo` → `hello`.

The test script `./scripts/test_examples.sh` (from repo root) prefers `./bin/blink-store`, then `dist/`, then target build.

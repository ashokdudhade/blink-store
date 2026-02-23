# Language guides

Client and backend examples are in `examples/clients/` for each language.

| Language | REPL client | HTTP backend |
|----------|-------------|--------------|
| Rust | `./dist/blink_client --tcp 127.0.0.1:8765` | `./dist/backend_http --store 127.0.0.1:8765 --port 8080` |
| Python | `python examples/clients/python/blink_client.py` | `BLINK_STORE=127.0.0.1:8765 python examples/clients/python/backend_app.py` |
| Node.js | `node examples/clients/node/blink_client.js` | `BLINK_STORE=127.0.0.1:8765 node examples/clients/node/backend_app.js` |
| Go | `go run examples/clients/go/blink_client.go` | `BLINK_STORE=127.0.0.1:8765 go run examples/clients/go/backend_app.go` |
| Shell | `bash examples/clients/shell/blink_client.sh` | — |

Integration notes (sockets, buffers, async): see [INTEGRATION_GUIDES.md](../../docs/INTEGRATION_GUIDES.md).

Quickstart: [examples/QUICKSTART.md](../../examples/QUICKSTART.md).

# Quickstart by language

Start the server first (from repo root):

```bash
./scripts/build-dist.sh && ./dist/blink-store serve --tcp 127.0.0.1 8765
```

Then in another terminal:

| Language | One-liner |
|----------|-----------|
| **Rust** | `./dist/blink_client --tcp 127.0.0.1:8765` |
| **Python** | `python examples/clients/python/blink_client.py` |
| **Node** | `node examples/clients/node/blink_client.js` |
| **Go** | `go run examples/clients/go/blink_client.go` |
| **Shell** | `bash examples/clients/shell/blink_client.sh` |

Try: `SET hello world`, `GET hello`, `USAGE`, `QUIT`.

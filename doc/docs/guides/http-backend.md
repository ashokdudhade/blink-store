---
title: HTTP Backend Pattern
---

# HTTP Backend Pattern

Use Blink Store as a cache behind your HTTP API. This pattern works in any language: your HTTP handler opens a TCP connection to Blink Store, sends a command, reads the response, and returns it to the HTTP client.

---

## Architecture

```text
┌──────────┐       HTTP        ┌───────────────┐       TCP        ┌─────────────┐
│  Client   │ ───────────────→ │  Your HTTP    │ ──────────────→ │ Blink Store │
│ (browser, │ ←─────────────── │  Backend      │ ←────────────── │   Server    │
│  curl)    │                  └───────────────┘                  └─────────────┘
└──────────┘
```

Your backend translates HTTP methods to Blink Store commands:

| HTTP | Blink Store | Behavior |
|------|------------|----------|
| `POST /<key>` (with body) | `SET key body` | Store a value |
| `GET /<key>` | `GET key` | Retrieve a value |
| `DELETE /<key>` | `DELETE key` | Remove a value |

---

## Python

```python
import base64, os, socket
from wsgiref.simple_server import make_server

STORE = os.environ.get("BLINK_STORE", "127.0.0.1:8765")

def blink(cmd):
    h, _, p = STORE.partition(":")
    with socket.socket() as s:
        s.connect((h, int(p)))
        s.sendall((cmd + "\n").encode())
        return s.recv(4096).decode().strip()

def app(environ, start_response):
    method = environ["REQUEST_METHOD"]
    key = (environ.get("PATH_INFO") or "").strip("/").split("/")[0]
    if not key:
        start_response("400 Bad Request", [])
        return [b"Missing key"]

    if method == "GET":
        r = blink(f"GET {key}")
        if r == "NOT_FOUND":
            start_response("404 Not Found", [])
            return [b"Not found"]
        if r.startswith("VALUE "):
            start_response("200 OK", [("Content-Type", "text/plain")])
            return [base64.b64decode(r[6:])]
        start_response("502 Bad Gateway", [])
        return [b"Store error"]

    if method == "POST":
        length = int(environ.get("CONTENT_LENGTH", 0) or 0)
        body = environ["wsgi.input"].read(length).decode()
        blink(f"SET {key} {body}")
        start_response("204 No Content", [])
        return []

    start_response("405 Method Not Allowed", [])
    return [b"Method not allowed"]

if __name__ == "__main__":
    port = int(os.environ.get("PORT", "8080"))
    print(f"HTTP backend on :{port} → store={STORE}")
    make_server("", port, app).serve_forever()
```

```bash
BLINK_STORE=127.0.0.1:8765 python3 backend.py
```

---

## Node.js

```javascript
const http = require('http');
const net = require('net');

const [storeHost, storePort] = (process.env.BLINK_STORE || '127.0.0.1:8765').split(':');
const httpPort = parseInt(process.env.PORT || '8080', 10);

function blink(cmd) {
  return new Promise((resolve, reject) => {
    const sock = net.createConnection(parseInt(storePort), storeHost, () => {
      sock.write(cmd + '\n');
    });
    let data = '';
    sock.on('data', (c) => { data += c.toString(); });
    sock.on('end', () => resolve(data.trim()));
    sock.on('error', reject);
  });
}

http.createServer(async (req, res) => {
  const key = (req.url || '/').slice(1).split('/')[0];
  if (!key) { res.writeHead(400); res.end('Missing key'); return; }

  if (req.method === 'GET') {
    const r = await blink(`GET ${key}`);
    if (r === 'NOT_FOUND') { res.writeHead(404); res.end('Not found'); return; }
    if (r.startsWith('VALUE ')) {
      res.writeHead(200, { 'Content-Type': 'text/plain' });
      res.end(Buffer.from(r.slice(6), 'base64'));
      return;
    }
    res.writeHead(502); res.end('Store error');
  } else if (req.method === 'POST') {
    let body = '';
    req.on('data', (c) => { body += c; });
    req.on('end', async () => {
      await blink(`SET ${key} ${body}`);
      res.writeHead(204); res.end();
    });
  } else {
    res.writeHead(405); res.end('Method not allowed');
  }
}).listen(httpPort, () => {
  console.log(`HTTP backend on :${httpPort} → store=${storeHost}:${storePort}`);
});
```

```bash
BLINK_STORE=127.0.0.1:8765 node backend.js
```

---

## Test any backend

Once running, test with `curl`:

```bash
# Store a value
curl -X POST http://localhost:8080/greeting -d 'hello world'

# Retrieve it
curl http://localhost:8080/greeting
# → hello world

# Delete it
curl -X DELETE http://localhost:8080/greeting
```

---

## Production tips

- **Connection pooling** — Open a persistent TCP connection to Blink Store instead of connecting per request. This reduces latency significantly under load.
- **Timeouts** — Set socket read/write timeouts to avoid hanging if the store is unresponsive.
- **Error handling** — Always handle `NOT_FOUND`, `ERROR`, and connection failures gracefully.
- **Health checks** — Use the `USAGE` command as a liveness probe.

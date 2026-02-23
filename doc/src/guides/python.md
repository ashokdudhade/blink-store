# Python

Connect to Blink-Store from Python using the standard library — no third-party packages needed.

---

## Prerequisites

- Python 3.6+
- Blink-Store server running ([Installation](../installation.md))

---

## Interactive client

A complete REPL client. Save as `client.py` and run it.

```python
#!/usr/bin/env python3
"""Blink-Store interactive client."""

import base64
import socket
import sys

def main():
    host = sys.argv[1] if len(sys.argv) > 1 else "127.0.0.1"
    port = int(sys.argv[2]) if len(sys.argv) > 2 else 8765

    with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as sock:
        sock.connect((host, port))
        reader = sock.makefile("r", encoding="utf-8", newline="\n")

        print(f"Connected to {host}:{port}")
        print("Commands: SET <key> <value> | GET <key> | DELETE <key> | USAGE | QUIT\n")

        try:
            while True:
                line = input("> ").strip()
                if not line:
                    continue
                if line.upper() == "QUIT":
                    break

                sock.sendall((line + "\n").encode())
                resp = reader.readline().strip()

                if resp.startswith("VALUE "):
                    decoded = base64.b64decode(resp[6:]).decode("utf-8", errors="replace")
                    print(decoded)
                else:
                    print(resp)
        finally:
            sock.sendall(b"QUIT\n")

if __name__ == "__main__":
    main()
```

**Run:**

```bash
python3 client.py
```

```text
Connected to 127.0.0.1:8765
Commands: SET <key> <value> | GET <key> | DELETE <key> | USAGE | QUIT

> SET user alice
OK
> GET user
alice
> USAGE
USAGE 9
> DELETE user
OK
> QUIT
```

---

## One-off commands

For scripting or quick lookups without an interactive session:

```python
import base64
import socket

def blink(command, host="127.0.0.1", port=8765):
    """Send a single command and return the response."""
    with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
        s.connect((host, port))
        s.sendall((command + "\n").encode())
        resp = s.recv(4096).decode().strip()
        if resp.startswith("VALUE "):
            return base64.b64decode(resp[6:]).decode()
        return resp

# Usage
blink("SET config.timeout 30")   # -> "OK"
blink("GET config.timeout")       # -> "30"
blink("USAGE")                     # -> "USAGE 18"
```

---

## HTTP backend example

Use Blink-Store as a cache behind a Python HTTP API. No dependencies beyond the standard library.

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
    print(f"HTTP backend on :{port} -> store={STORE}")
    make_server("", port, app).serve_forever()
```

**Run and test:**

```bash
BLINK_STORE=127.0.0.1:8765 python3 backend.py &

curl -X POST http://localhost:8080/greeting -d 'hello world'
curl http://localhost:8080/greeting
# -> hello world
```

---

## Key concepts

| Concept | Python API |
|---------|-----------|
| TCP connection | `socket.socket(AF_INET, SOCK_STREAM)` |
| Line reading | `sock.makefile("r").readline()` |
| Base64 decode | `base64.b64decode(payload)` |
| Encoding | All strings are UTF-8 |

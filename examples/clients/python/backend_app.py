# Minimal HTTP backend using Blink-Store. GET /<key>, POST /<key> with body.
# Start store: cargo run -- serve --tcp 127.0.0.1:8765
# Run: BLINK_STORE=127.0.0.1:8765 python backend_app.py

import base64
import os
import socket
from wsgiref.simple_server import make_server

def store_request(cmd, key, value=b""):
    addr = os.environ.get("BLINK_STORE", "127.0.0.1:8765")
    h, _, p = addr.partition(":")
    p = int(p or "8765")
    with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
        s.connect((h, p))
        req = f"SET {key} {value.decode('utf-8', errors='replace')}\n" if value else f"{cmd} {key}\n"
        s.sendall(req.encode("utf-8"))
        return s.recv(4096).decode("utf-8").strip()

def app(environ, start_response):
    method, path = environ["REQUEST_METHOD"], (environ.get("PATH_INFO") or "").strip("/")
    if not path: start_response("400 Bad Request", []); return [b"Use /<key>"]
    key = path.split("/")[0]
    if method == "GET":
        r = store_request("GET", key)
        if r == "NOT_FOUND": start_response("404 Not Found", []); return [b"Not Found"]
        if r.startswith("VALUE "):
            try:
                start_response("200 OK", [("Content-Type", "application/octet-stream")])
                return [base64.b64decode(r[6:].strip())]
            except Exception: pass
        start_response("502 Bad Gateway", []); return [b"Bad Gateway"]
    if method == "POST":
        try:
            cl = int(environ.get("CONTENT_LENGTH", "0") or "0")
        except ValueError:
            cl = 0
        body = environ["wsgi.input"].read(cl) if cl else b""
        r = store_request("SET", key, body)
        if r.startswith("OK"): start_response("204 No Content", []); return []
        start_response("502 Bad Gateway", []); return [b"Bad Gateway"]
    start_response("405 Method Not Allowed", []); return [b"Method Not Allowed"]

if __name__ == "__main__":
    port = int(os.environ.get("PORT", "8080"))
    with make_server("", port, app) as h:
        print("Backend http://0.0.0.0:%d store=%s" % (port, os.environ.get("BLINK_STORE", "127.0.0.1:8765")))
        h.serve_forever()

---
sidebar_position: 3
title: Protocol Reference
---

# Protocol Reference

Blink-Store uses a **line-based text protocol** over TCP or Unix sockets.

- One command per line, one response per line.
- Encoding: UTF-8 with `\n` (LF) line endings.
- Values returned by `GET` are **base64-encoded**.

---

## Commands

| Command | Arguments | Description |
|---------|-----------|-------------|
| `SET`   | `key value` | Store a value. The value is everything after the first space following the key. |
| `GET`   | `key`       | Retrieve a value. Returns base64-encoded bytes. |
| `DELETE`| `key`       | Remove a key. |
| `USAGE` | *(none)*    | Return the current memory usage in bytes. |
| `QUIT`  | *(none)*    | Close the connection gracefully. |

---

## Responses

| Response | Meaning |
|----------|---------|
| `OK`          | Command succeeded (`SET`, `DELETE`). |
| `VALUE <b64>` | Key found. `<b64>` is the base64-encoded value. |
| `NOT_FOUND`   | Key does not exist. |
| `USAGE <n>`   | Current stored size in bytes (keys + values). |
| `ERROR <msg>` | Something went wrong. `<msg>` describes the error. |

---

## Example session

Below is a complete interaction over a raw TCP connection. Lines starting with `>` are sent by the client; lines starting with `<` are responses from the server.

```text
> GET mykey
< NOT_FOUND

> SET mykey hello
< OK

> GET mykey
< VALUE aGVsbG8=

> SET counter 42
< OK

> USAGE
< USAGE 18

> DELETE mykey
< OK

> GET mykey
< NOT_FOUND

> QUIT
```

Decoding the base64 value: `aGVsbG8=` → `hello`.

---

## Transport

| Flag | Transport | Platforms |
|------|-----------|-----------|
| `--tcp <host>:<port>` | TCP socket | All |
| `--unix <path>` | Unix domain socket | Linux, macOS |

The server can listen on TCP, Unix, or both simultaneously.

---

## Connecting with common tools

**netcat:**

```bash
echo "SET foo bar" | nc 127.0.0.1 8765
echo "GET foo"     | nc 127.0.0.1 8765
```

**Python (one-off):**

```python
import socket
s = socket.socket()
s.connect(("127.0.0.1", 8765))
s.sendall(b"SET foo bar\n")
print(s.recv(1024).decode())   # OK
s.sendall(b"GET foo\n")
print(s.recv(1024).decode())   # VALUE YmFy
s.close()
```

**bash `/dev/tcp`:**

```bash
exec 3<>/dev/tcp/127.0.0.1/8765
echo "SET foo bar" >&3
read -r reply <&3; echo "$reply"   # OK
exec 3<&-
```

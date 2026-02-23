---
sidebar_position: 4
title: Language Guides
slug: /guides/
---

# Language Guides

Blink Store speaks a plain-text [protocol](../protocol) over TCP. Any language with socket support can connect — no SDK, no driver, no library to install.

Each guide below includes a complete, copy-paste interactive client, a one-off command helper, and language-specific tips.

---

## Choose your language

| Language | Guide | Dependencies |
|----------|-------|-------------|
| **Python** | [Python guide](python) | Standard library only |
| **Node.js** | [Node.js guide](nodejs) | Standard library only |
| **Go** | [Go guide](go) | Standard library only |
| **Shell** | [Shell (Bash) guide](shell) | `bash` + `base64` |
| **Rust** | [Rust guide](rust) | `base64` crate |

---

## HTTP backends

Want to use Blink Store as a cache behind your HTTP API? See the [HTTP Backend Pattern](http-backend) for complete server examples in Python, Node.js, Go, and Rust.

---

## Any other language?

Blink Store works with any language that can:

1. Open a TCP socket
2. Send a line of text (UTF-8, `\n` terminated)
3. Read a line of text back
4. Decode base64

See the [Protocol Reference](../protocol) for the full specification. Here's the pattern in pseudocode:

```text
sock = tcp_connect("127.0.0.1", 8765)
sock.send("SET mykey hello\n")
response = sock.readline()          // "OK"

sock.send("GET mykey\n")
response = sock.readline()          // "VALUE aGVsbG8="
value = base64_decode("aGVsbG8=")   // "hello"
```

| Language | Socket API | Line reading | Base64 decode |
|----------|-----------|-------------|---------------|
| Python   | `socket.socket()` | `.makefile('r').readline()` | `base64.b64decode()` |
| Node.js  | `net.createConnection()` | Buffer + split on `\n` | `Buffer.from(b64, 'base64')` |
| Go       | `net.Dial("tcp", addr)` | `bufio.ReadString('\n')` | `base64.StdEncoding.DecodeString()` |
| Rust     | `TcpStream::connect()` | `BufReader::read_line()` | `base64::decode()` |
| Java     | `new Socket(host, port)` | `BufferedReader.readLine()` | `Base64.getDecoder().decode()` |
| C#       | `TcpClient` | `StreamReader.ReadLine()` | `Convert.FromBase64String()` |
| Ruby     | `TCPSocket.new(host, port)` | `.gets` | `Base64.decode64()` |
| PHP      | `fsockopen(host, port)` | `fgets()` | `base64_decode()` |

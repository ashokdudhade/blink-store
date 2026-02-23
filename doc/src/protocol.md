# Protocol

Blink-Store uses a **line-based text protocol** over TCP or Unix socket. One request per line, one response per line. UTF-8, LF line endings.

## Commands (client to server)

| Command | Description | Example |
|---------|-------------|---------|
| GET key | Read value | GET foo |
| SET key value | Write (value = rest of line) | SET foo bar |
| DELETE key | Remove key | DELETE foo |
| USAGE | Current byte usage | USAGE |
| QUIT | Close connection | QUIT |

## Responses (server to client)

| Response | Meaning |
|----------|--------|
| OK | Success (SET, DELETE) |
| VALUE base64 | GET value; decode base64 to bytes |
| NOT_FOUND | Key missing |
| USAGE n | Current size in bytes |
| ERROR msg | Error message |

## Example session

```
GET x
NOT_FOUND

SET x hello
OK

GET x
VALUE aGVsbG8=

USAGE
USAGE 8

DELETE x
OK

QUIT
```

Values in GET are base64-encoded. Transport: TCP with `--tcp`, or Unix socket with `--unix` (Unix only). Full spec: repo `docs/PROTOCOL_SPEC.md`.

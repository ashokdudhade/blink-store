# Blink-Store Protocol Specification

Transport: TCP or Unix domain socket. Encoding: UTF-8, LF line endings.

## Line-Based Text Protocol

One request per line; one response per line.

### Commands

| Command | Description |
|---------|-------------|
| GET key | Read value |
| SET key value | Write (value = rest of line) |
| DELETE key | Remove key |
| DEL key | Alias for DELETE |
| USAGE | Current byte usage |
| STATS | Alias for USAGE |
| QUIT | Close connection |

### Responses

| Line | Meaning |
|------|--------|
| OK | Success |
| VALUE base64 | GET value (base64) |
| NOT_FOUND | Key missing |
| USAGE n | Bytes used |
| ERROR msg | Error |

### Future: Binary Protocol

Optional extension: 4-byte length header (big-endian) + payload (command + key + value). EXPIRE/TTL not yet implemented.

See protocol.md and src/protocol.rs.

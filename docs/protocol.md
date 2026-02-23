# Blink-Store network protocol

Line-based text protocol over TCP or Unix socket. UTF-8, LF (`\n`) line endings.

## Commands (client → server)

| Command   | Description              | Example           |
|----------|---------------------------|-------------------|
| `GET <key>` | Read value for key     | `GET foo`         |
| `SET <key> <value>` | Write value (rest of line) | `SET foo bar` |
| `DELETE <key>` | Remove key            | `DELETE foo`      |
| `USAGE`  | Get current byte usage     | `USAGE`           |
| `QUIT`   | Close connection            | `QUIT`            |

## Responses (server → client)

| Line        | Meaning                    |
|-------------|----------------------------|
| `OK`        | Success (SET, DELETE)      |
| `VALUE <base64>` | Value (GET); decode base64 to bytes |
| `NOT_FOUND` | Key missing (GET, DELETE)   |
| `USAGE <n>` | Current usage in bytes      |
| `ERROR <msg>` | Error message             |

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

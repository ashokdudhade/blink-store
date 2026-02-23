---
title: Shell (Bash)
---

# Shell (Bash)

Connect to Blink-Store from bash using built-in `/dev/tcp` — no external tools needed beyond `base64`.

---

## Prerequisites

- Bash 4+
- `base64` command (pre-installed on most systems)
- Blink-Store server running ([Installation](../installation))

---

## Interactive client

A complete REPL client. Save as `client.sh` and run it.

```bash
#!/usr/bin/env bash
set -euo pipefail

HOST="${1:-127.0.0.1}"
PORT="${2:-8765}"

exec 3<>/dev/tcp/"$HOST"/"$PORT" 2>/dev/null || {
  echo "Cannot connect to $HOST:$PORT" >&2
  exit 1
}

echo "Connected to $HOST:$PORT"
echo "Commands: SET <key> <value> | GET <key> | DELETE <key> | USAGE | QUIT"
echo

while printf "> " && read -r line; do
  [ -z "$line" ] && continue
  [[ "${line^^}" == "QUIT" ]] && break

  printf '%s\n' "$line" >&3
  read -r -u 3 resp

  if [[ "$resp" == VALUE\ * ]]; then
    echo "${resp#VALUE }" | base64 -d 2>/dev/null
    echo
  else
    echo "$resp"
  fi
done

printf 'QUIT\n' >&3
exec 3<&-
```

**Run:**

```bash
bash client.sh
```

```text
Connected to 127.0.0.1:8765
Commands: SET <key> <value> | GET <key> | DELETE <key> | USAGE | QUIT

> SET name bash
OK
> GET name
bash
> QUIT
```

---

## One-off commands

Send a single command from a script:

```bash
#!/usr/bin/env bash
# blink.sh -- send one command to Blink-Store
HOST="${2:-127.0.0.1}"
PORT="${3:-8765}"

exec 3<>/dev/tcp/"$HOST"/"$PORT"
printf '%s\n' "$1" >&3
read -r resp <&3
exec 3<&-

if [[ "$resp" == VALUE\ * ]]; then
  echo "${resp#VALUE }" | base64 -d
else
  echo "$resp"
fi
```

**Usage:**

```bash
bash blink.sh "SET key value"   # -> OK
bash blink.sh "GET key"          # -> value
bash blink.sh "USAGE"            # -> USAGE 8
```

---

## Using netcat

If `/dev/tcp` is not available (e.g. on minimal containers), use `nc`:

```bash
echo "SET color red" | nc 127.0.0.1 8765     # -> OK
echo "GET color"     | nc 127.0.0.1 8765     # -> VALUE cmVk
echo "cmVk" | base64 -d                       # -> red
```

---

## Key concepts

| Concept | Bash API |
|---------|---------|
| TCP connection | `exec 3<>/dev/tcp/HOST/PORT` |
| Send command | `printf '%s\n' "$cmd" >&3` |
| Read response | `read -r -u 3 resp` |
| Base64 decode | `echo "$b64" \| base64 -d` |
| Close socket | `exec 3<&-` |

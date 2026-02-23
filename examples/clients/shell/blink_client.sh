#!/usr/bin/env bash
# Example Blink Store client (TCP). Requires: bash, base64.
# Usage: ./blink_client.sh [host [port]]
#        ./blink_client.sh 127.0.0.1 8765

HOST="${1:-127.0.0.1}"
PORT="${2:-8765}"

if ! command -v base64 &>/dev/null; then
  echo "error: base64 required" >&2
  exit 1
fi

exec 3<>/dev/tcp/"$HOST"/"$PORT" 2>/dev/null || {
  echo "error: cannot connect to $HOST:$PORT (try: bash, or use nc)" >&2
  exit 1
}

echo "Commands: GET <key> | SET <key> <value> | DELETE <key> | USAGE | QUIT"

while read -r line; do
  line="${line%%#*}"
  line="$(printf '%s' "$line" | tr -d '\r')"
  [ -z "$line" ] && continue
  cmd="${line%% *}"
  [ "$(printf '%s' "$cmd" | tr '[:lower:]' '[:upper:]')" = "QUIT" ] && break
  printf '%s\n' "$line" >&3
  read -r -u 3 resp
  resp="${resp%%$'\r'*}"
  if [ "${resp#VALUE }" != "$resp" ]; then
    b64="${resp#VALUE }"
    b64="${b64%% *}"
    printf '%s' "$b64" | base64 -d 2>/dev/null || echo "$resp"
    echo
  else
    printf '%s\n' "$resp"
  fi
done

printf 'QUIT\n' >&3
exec 3<&-

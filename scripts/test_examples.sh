#!/usr/bin/env bash
# Quick test that all REPL clients and backends work. Run from repo root.
# Prefers: ./bin/blink-store (from install-from-github.sh), then dist/, then target build.
set -e
cd "$(dirname "$0")/.."
BLINK_PORT=39999
HTTP_PORT=39998

# Prefer GitHub-installed server, then dist, then target; Rust client/backend only in dist or target
if [[ -x ./bin/blink-store ]] || [[ -f ./bin/blink-store.exe ]]; then
  BLINK_BIN=./bin/blink-store
  [[ -f ./bin/blink-store.exe ]] && BLINK_BIN=./bin/blink-store.exe
fi
if [[ -z "$BLINK_BIN" ]] && { [[ -x ./dist/blink-store ]] || [[ -f ./dist/blink-store.exe ]]; }; then
  BLINK_BIN=./dist/blink-store
  [[ -f ./dist/blink-store.exe ]] && BLINK_BIN=./dist/blink-store.exe
fi
if [[ -x ./dist/blink_client ]] || [[ -f ./dist/blink_client.exe ]]; then
  RUST_CLIENT=./dist/blink_client
  RUST_BACKEND=./dist/backend_http
  [[ -f ./dist/blink_client.exe ]] && RUST_CLIENT=./dist/blink_client.exe
  [[ -f ./dist/backend_http.exe ]] && RUST_BACKEND=./dist/backend_http.exe
elif [[ -x ./target/release/blink-store ]] || [[ -f ./target/release/blink-store.exe ]]; then
  [[ -z "$BLINK_BIN" ]] && BLINK_BIN=./target/release/blink-store
  [[ -z "$BLINK_BIN" ]] && [[ -f ./target/release/blink-store.exe ]] && BLINK_BIN=./target/release/blink-store.exe
  RUST_CLIENT=./target/release/examples/blink_client
  RUST_BACKEND=./target/release/examples/backend_http
  [[ -f ./target/release/examples/blink_client.exe ]] && RUST_CLIENT=./target/release/examples/blink_client.exe
  [[ -f ./target/release/examples/backend_http.exe ]] && RUST_BACKEND=./target/release/examples/backend_http.exe
fi
if [[ -z "$BLINK_BIN" ]]; then
  BLINK_BIN=./target/release/blink-store
  RUST_CLIENT=./target/release/examples/blink_client
  RUST_BACKEND=./target/release/examples/backend_http
  [[ -f ./target/release/blink-store.exe ]] && BLINK_BIN=./target/release/blink-store.exe
  [[ -f ./target/release/examples/blink_client.exe ]] && RUST_CLIENT=./target/release/examples/blink_client.exe
  [[ -f ./target/release/examples/backend_http.exe ]] && RUST_BACKEND=./target/release/examples/backend_http.exe
fi
if [[ -z "$RUST_CLIENT" ]] && { [[ -x ./target/release/blink-store ]] || [[ -f ./target/release/blink-store.exe ]]; }; then
  RUST_CLIENT=./target/release/examples/blink_client
  RUST_BACKEND=./target/release/examples/backend_http
  [[ -f ./target/release/examples/blink_client.exe ]] && RUST_CLIENT=./target/release/examples/blink_client.exe
  [[ -f ./target/release/examples/backend_http.exe ]] && RUST_BACKEND=./target/release/examples/backend_http.exe
elif [[ -z "$RUST_CLIENT" ]]; then
  RUST_CLIENT=./target/debug/examples/blink_client
  RUST_BACKEND=./target/debug/examples/backend_http
  [[ -f ./target/debug/examples/blink_client.exe ]] && RUST_CLIENT=./target/debug/examples/blink_client.exe
  [[ -f ./target/debug/examples/backend_http.exe ]] && RUST_BACKEND=./target/debug/examples/backend_http.exe
fi
if [[ -z "$BLINK_BIN" ]]; then
  BLINK_BIN=./target/release/blink-store
  [[ -f ./target/release/blink-store.exe ]] && BLINK_BIN=./target/release/blink-store.exe
fi
if [[ -z "$RUST_CLIENT" ]]; then
  RUST_CLIENT=./target/release/examples/blink_client
  RUST_BACKEND=./target/release/examples/backend_http
  [[ -f ./target/release/examples/blink_client.exe ]] && RUST_CLIENT=./target/release/examples/blink_client.exe
  [[ -f ./target/release/examples/backend_http.exe ]] && RUST_BACKEND=./target/release/examples/backend_http.exe
fi
if [[ -z "$BLINK_BIN" ]] || [[ -z "$RUST_CLIENT" ]]; then
  BLINK_BIN=./target/debug/blink-store
  RUST_CLIENT=./target/debug/examples/blink_client
  RUST_BACKEND=./target/debug/examples/backend_http
  [[ -f ./target/debug/blink-store.exe ]] && BLINK_BIN=./target/debug/blink-store.exe
  [[ -f ./target/debug/examples/blink_client.exe ]] && RUST_CLIENT=./target/debug/examples/blink_client.exe
  [[ -f ./target/debug/examples/backend_http.exe ]] && RUST_BACKEND=./target/debug/examples/backend_http.exe
fi

cleanup() {
  pkill -f "blink-store serve" 2>/dev/null || true
  pkill -f "backend_http" 2>/dev/null || true
  pkill -f "backend_app" 2>/dev/null || true
  sleep 1
}
trap cleanup EXIT
cleanup

echo "Starting Blink-Store on $BLINK_PORT (using $BLINK_BIN)..."
"$BLINK_BIN" serve --tcp 127.0.0.1:$BLINK_PORT 2>/dev/null &
sleep 1

ok=0
fail=0

# REPL clients: Rust (cargo), examples/clients (python, node, go, shell)
for name in rust python node go shell; do
  case $name in
    rust)  cmd="printf 'SET t ok\nGET t\nQUIT\n' | $RUST_CLIENT --tcp 127.0.0.1:$BLINK_PORT 2>/dev/null" ; label="Rust (cargo)" ;;
    python) cmd="printf 'SET t ok\nGET t\nQUIT\n' | python3 examples/clients/python/blink_client.py 127.0.0.1 $BLINK_PORT 2>/dev/null" ; label="examples/clients/python" ;;
    node)  cmd="printf 'SET t ok\nGET t\nQUIT\n' | node examples/clients/node/blink_client.js 127.0.0.1 $BLINK_PORT 2>/dev/null" ; label="examples/clients/node" ;;
    go)    cmd="printf 'SET t ok\nGET t\nQUIT\n' | go run examples/clients/go/blink_client.go 127.0.0.1 $BLINK_PORT 2>/dev/null" ; label="examples/clients/go" ;;
    shell) cmd="printf 'SET t ok\nGET t\nQUIT\n' | bash examples/clients/shell/blink_client.sh 127.0.0.1 $BLINK_PORT 2>/dev/null" ; label="examples/clients/shell" ;;
  esac
  if out=$(eval "$cmd") && echo "$out" | grep -q "ok"; then
    echo "  REPL $name ($label): OK"
    ((ok++)) || true
  else
    echo "  REPL $name ($label): FAIL"
    ((fail++)) || true
  fi
done

# Backends: Rust (cargo), examples/clients (python, node, go)
"$RUST_BACKEND" --store 127.0.0.1:$BLINK_PORT --port $HTTP_PORT 2>/dev/null &
sleep 1
if curl -s -X POST http://127.0.0.1:$HTTP_PORT/x -d "rust" >/dev/null && [ "$(curl -s http://127.0.0.1:$HTTP_PORT/x)" = "rust" ]; then
  echo "  Backend Rust (cargo): OK"
  ((ok++)) || true
else
  echo "  Backend Rust (cargo): FAIL"
  ((fail++)) || true
fi
pkill -f "backend_http" 2>/dev/null || true
sleep 1

# examples/clients/python backend (wsgiref starts slowly; wait until port responds)
PY_PORT=$((HTTP_PORT-1))
BLINK_STORE=127.0.0.1:$BLINK_PORT PORT=$PY_PORT timeout 15 python3 examples/clients/python/backend_app.py 2>/dev/null &
i=0; until curl -s -o /dev/null --connect-timeout 1 http://127.0.0.1:$PY_PORT/ 2>/dev/null || [ "$i" -ge 30 ]; do sleep 0.3; i=$((i+1)); done
sleep 0.2
if curl -s -X POST http://127.0.0.1:$PY_PORT/x -d "python" >/dev/null && [ "$(curl -s http://127.0.0.1:$PY_PORT/x)" = "python" ]; then
  echo "  Backend Python (examples/clients/python): OK"
  ((ok++)) || true
else
  echo "  Backend Python (examples/clients/python): FAIL"
  ((fail++)) || true
fi
pkill -f "backend_app.py" 2>/dev/null || true
sleep 1
unset PY_PORT

# examples/clients/node backend
BLINK_STORE=127.0.0.1:$BLINK_PORT PORT=$((HTTP_PORT-2)) timeout 5 node examples/clients/node/backend_app.js 2>/dev/null &
sleep 1
if curl -s -X POST http://127.0.0.1:$((HTTP_PORT-2))/x -d "node" >/dev/null && [ "$(curl -s http://127.0.0.1:$((HTTP_PORT-2))/x)" = "node" ]; then
  echo "  Backend Node (examples/clients/node): OK"
  ((ok++)) || true
else
  echo "  Backend Node (examples/clients/node): FAIL"
  ((fail++)) || true
fi
pkill -f "backend_app.js" 2>/dev/null || true
sleep 1

# examples/clients/go backend
BLINK_STORE=127.0.0.1:$BLINK_PORT PORT=$((HTTP_PORT-3)) timeout 8 go run examples/clients/go/backend_app.go 2>/dev/null &
sleep 3
if curl -s -X POST http://127.0.0.1:$((HTTP_PORT-3))/x -d "go" >/dev/null && [ "$(curl -s http://127.0.0.1:$((HTTP_PORT-3))/x)" = "go" ]; then
  echo "  Backend Go (examples/clients/go): OK"
  ((ok++)) || true
else
  echo "  Backend Go (examples/clients/go): FAIL"
  ((fail++)) || true
fi

echo ""
echo "Total: $ok passed, $fail failed"
[ "$fail" -eq 0 ]

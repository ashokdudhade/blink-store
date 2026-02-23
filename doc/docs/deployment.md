---
sidebar_position: 5
title: Deployment
---

# Deployment

This page covers running Blink-Store in production: standalone, Docker, resource limits, and logging.

---

## Standalone

Install the latest binary (see [Installation](installation)) and run:

```bash
./blink-store serve --tcp 0.0.0.0:8765 --memory-limit 10485760
```

| Flag | Default | Description |
|------|---------|-------------|
| `--tcp <addr>:<port>` | *required* | TCP listen address. Use `0.0.0.0` to accept external connections. |
| `--unix <path>` | *(none)* | Unix domain socket path (Linux/macOS only). |
| `--memory-limit <bytes>` | `10485760` (10 MiB) | Maximum total size of stored keys + values. LRU eviction kicks in when exceeded. |
| `--log-dir <path>` | *(none)* | Directory for rolling log files. |
| `--retention-minutes <n>` | *(none)* | Auto-delete log files older than *n* minutes. |

---

## Docker

### Quick start

```bash
docker run -d --name blink-store \
  -p 8765:8765 \
  ghcr.io/ashokdudhade/blink-store:latest
```

Override defaults with environment variables:

```bash
docker run -d --name blink-store \
  -p 9000:9000 \
  -e BLINK_PORT=9000 \
  -e BLINK_MEMORY_LIMIT=104857600 \
  -e BLINK_RETENTION_MINUTES=1440 \
  ghcr.io/ashokdudhade/blink-store:latest
```

Or build from a Dockerfile (no Git clone):

```bash
curl -sSLf -o Dockerfile \
  https://raw.githubusercontent.com/ashokdudhade/blink-store/main/Dockerfile
docker build -t blink-store .
docker run -d -p 8765:8765 blink-store serve --tcp 0.0.0.0:8765
```

### Docker Compose

```yaml
services:
  blink-store:
    build: .
    command: ["serve", "--tcp", "0.0.0.0:8765", "--memory-limit", "10485760"]
    ports:
      - "8765:8765"
    deploy:
      resources:
        limits:
          memory: 32M
          cpus: "0.5"
        reservations:
          memory: 8M
    restart: unless-stopped
```

```bash
docker compose up -d
```

---

## Resource limits

### Memory

| Layer | Flag / setting | Purpose |
|-------|---------------|---------|
| Application | `--memory-limit <bytes>` | Caps total stored data. Triggers LRU eviction. |
| Container | `deploy.resources.limits.memory` | Hard cap on the container. Prevents OOM-kill from affecting the host. |

Set the container limit higher than `--memory-limit` to account for process overhead (stack, buffers, etc.). A ratio of **3:1** is a safe starting point — e.g. `--memory-limit 10485760` (10 MiB) with a container limit of `32M`.

### CPU

```yaml
deploy:
  resources:
    limits:
      cpus: "0.5"
```

Or use `cpuset` for core pinning in performance-sensitive scenarios.

---

## Logging

Enable structured logging to files with automatic rotation:

```bash
./blink-store serve --tcp 0.0.0.0:8765 \
  --memory-limit 10485760 \
  --log-dir /var/log/blink-store \
  --retention-minutes 1440
```

| Flag | Description |
|------|-------------|
| `--log-dir` | Directory for rolling log files. Created if it doesn't exist. |
| `--retention-minutes` | Prune log files older than this. `1440` = 24 hours. |

Logs use structured `tracing` format and include timestamps, log levels, key names, and byte sizes for every operation.

---

## Health check

Blink-Store responds to `USAGE` with the current memory usage:

```bash
echo "USAGE" | nc 127.0.0.1 8765
# → USAGE 1234
```

Use this as a TCP health check in Docker, Kubernetes, or your load balancer. A response confirms the server is alive and accepting connections.

---

## Production checklist

- [ ] Set `--memory-limit` based on available RAM
- [ ] Set container memory limits (Docker / K8s)
- [ ] Enable `--log-dir` with `--retention-minutes`
- [ ] Bind to `0.0.0.0` only if external access is needed; use `127.0.0.1` for local-only
- [ ] Pin the binary version (e.g. `v0.1.0`) in production deployments instead of `latest`
- [ ] Add a health check on the `USAGE` command

# Deployment

## Docker

Without cloning: download the Dockerfile and build:

```bash
docker run -p 8765:8765 ghcr.io/ashokdudhade/blink-store:latest
```

Or build locally (no clone):

```bash
curl -sSLf https://raw.githubusercontent.com/ashokdudhade/blink-store/main/Dockerfile -o Dockerfile
docker build -t blink-store .
docker run -p 8765:8765 blink-store
```

With the repo: `docker compose up -d`. See `docker-compose.yaml` for memory limits (default 32M cap, 8M reserved). Adjust `deploy.resources.limits.memory` and `reservations.memory` as needed.

## Resource limits

- **Memory**: Use `--memory-limit` (bytes) to cap store size. In Docker, set `deploy.resources.limits.memory` to avoid container OOM.
- **CPU**: Optional `cpuset` or `cpus` in docker-compose for pinning.

## Install with curl (no clone, latest)

```bash
curl -sSLf https://raw.githubusercontent.com/ashokdudhade/blink-store/main/scripts/install-from-github.sh | bash -s -- latest ./bin
./bin/blink-store serve --tcp 0.0.0.0:8765 --memory-limit 10485760
```

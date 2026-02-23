# Deployment

## Docker Compose

```yaml
# docker-compose.yaml (see repo root)
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
```

Run: `docker compose up -d`.

## Resource limits

- **Memory**: Use `--memory-limit` (bytes) to cap store size. In Docker, set `deploy.resources.limits.memory` to avoid OOM.
- **CPU**: Optional `cpuset` or `cpus` in docker-compose for pinning.

Details: [DEPLOYMENT.md](../../docs/DEPLOYMENT.md).

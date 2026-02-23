# Deployment

## Docker

```bash
docker compose up -d
# Server on host:8765
```

See `docker-compose.yaml` for memory limits (default 32M cap, 8M reserved). Adjust `deploy.resources.limits.memory` and `reservations.memory` as needed.

## Resource limits

- **Memory**: Use `--memory-limit` (bytes) to cap store size. In Docker, set `deploy.resources.limits.memory` to avoid container OOM.
- **CPU**: Optional `cpuset` or `cpus` in docker-compose for pinning.

## Local distribution

```bash
./scripts/build-dist.sh
./dist/blink-store serve --tcp 0.0.0.0:8765 --memory-limit 10485760
```

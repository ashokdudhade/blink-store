# Deployment

This page covers running Blink-Store in a deployed environment: Docker Compose and resource limits.

## Docker Compose

From the repository root, a typical setup is:

```yaml
# docker-compose.yaml (excerpt)
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
        reservations:
          memory: 8M
```

Then run:

```bash
docker compose up -d
```

Clients connect to the host on port **8765**. Adjust `--memory-limit` (bytes) to match how much RAM you want the store to use.

## Resource limits

### Memory

- **Application** — `--memory-limit` caps the total size of stored keys and values (in bytes). Example: `10485760` = 10 MiB.
- **Container** — In Docker or Kubernetes, set a memory limit on the container (e.g. `deploy.resources.limits.memory: 32M`) so the process cannot exceed it and get OOM-killed unexpectedly.

### CPU

You can pin the process to specific CPUs in Compose:

```yaml
deploy:
  resources:
    limits:
      cpus: "0.5"
```

Or use `cpuset` for pinning to particular cores.

## Logging and retention

- Use `--log-dir /path/to/logs` to write rolling log files.
- Use `--retention-minutes 60` so a background task prunes log files older than 60 minutes.

Example (install server first: `./scripts/install-from-github.sh ./bin`):

```bash
./bin/blink-store serve --tcp 0.0.0.0:8765 \
  --memory-limit 10485760 \
  --log-dir /var/log/blink-store \
  --retention-minutes 1440
```

For more examples and a full `docker-compose.yaml`, see the repo: `docs/DEPLOYMENT.md`.

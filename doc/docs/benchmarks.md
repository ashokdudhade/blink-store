---
sidebar_position: 6
title: Benchmarks
---

# Benchmarks

Measured on a single persistent TCP connection, sequential operations, 10,000 operations per test. Server configured with `--memory-limit 2097152` (2 MiB).

:::info Test environment
| | |
|:--|:--|
| **CPU** | Intel Core 7 150U — 6 cores / 12 threads |
| **Memory** | 7.6 GiB (WSL2 allocation) |
| **OS** | Ubuntu 24.04 LTS on WSL2 |
| **Kernel** | 6.6.87.2-microsoft-standard-WSL2 |
| **Arch** | x86_64 |

Results will vary by hardware and network stack.
:::

---

## Native (no container)

Server runs directly on the host. All cores and memory available.

### Throughput

| Operation | Value size | ops/sec |
|-----------|-----------|---------|
| `SET` | 64 bytes | ~8,950 |
| `SET` | 256 bytes | ~9,600 |
| `SET` | 1 KiB | ~8,800 |
| `GET` | — | ~11,100 |
| `DELETE` | — | ~9,700 |

### Latency

| Operation | avg | p50 | p95 | p99 | max |
|-----------|-----|-----|-----|-----|-----|
| `SET` | 106 us | 85 us | 207 us | 347 us | 5.97 ms |
| `GET` | 91 us | 81 us | 151 us | 231 us | 906 us |

---

## Container (1 CPU, 2 GB memory)

Server runs inside a container pulled from `ghcr.io/ashokdudhade/blink-store:latest`, restricted to 1 CPU and 2 GB memory via Podman/Docker resource limits.

```bash
podman run -d --name blink-store \
  --cpus 1 --memory 2g \
  -p 8765:8765 \
  -e BLINK_MEMORY_LIMIT=2097152 \
  ghcr.io/ashokdudhade/blink-store:latest
```

### Throughput

| Operation | Value size | ops/sec |
|-----------|-----------|---------|
| `SET` | 64 bytes | ~4,430 |
| `SET` | 256 bytes | ~4,930 |
| `SET` | 1 KiB | ~4,100 |
| `GET` | — | ~5,730 |
| `DELETE` | — | ~5,900 |

### Latency

| Operation | avg | p50 | p95 | p99 | max |
|-----------|-----|-----|-----|-----|-----|
| `SET` | 204 us | 180 us | 328 us | 455 us | 966 us |
| `GET` | 185 us | 167 us | 294 us | 398 us | 959 us |

### Native vs. container comparison

| Metric | Native | Container (1 CPU, 2 GB) | Overhead |
|--------|--------|------------------------|----------|
| SET 256B throughput | 9,600 ops/s | 4,930 ops/s | ~49% |
| GET throughput | 11,100 ops/s | 5,730 ops/s | ~48% |
| SET p50 latency | 85 us | 180 us | ~2.1x |
| GET p50 latency | 81 us | 167 us | ~2.1x |
| SET p95 latency | 207 us | 328 us | ~1.6x |
| GET p95 latency | 151 us | 294 us | ~1.9x |
| SET p99 latency | 347 us | 455 us | ~1.3x |
| GET p99 latency | 231 us | 398 us | ~1.7x |

The container overhead comes from two layers: the WSL2 virtual network adapter and the container network namespace. On bare-metal Linux with Docker, expect significantly less overhead.

---

## Memory cap enforcement

Proves that `--memory-limit` is respected and LRU eviction works correctly. Results are identical in both native and container runs.

**Setup:** 2 MiB memory limit, inserting 4,000 keys with 1 KiB values each (~4 MiB of data into a 2 MiB store).

| Metric | Result |
|--------|--------|
| Keys inserted | 4,000 |
| Data attempted | ~4 MiB |
| Memory limit | 2,097,152 bytes (2 MiB) |
| Final usage | 2,096,790 bytes (2,048 KiB) |
| Usage vs. limit | **99.98%** — right at the boundary, never exceeded |

### LRU eviction verification

| Key | Status | Explanation |
|-----|--------|-------------|
| `key_1` | evicted | Oldest key, first to be reclaimed |
| `key_2` | evicted | |
| `key_3` | evicted | |
| `key_10` | evicted | |
| `key_50` | evicted | |
| `key_100` | evicted | Still old enough to be evicted |
| `key_3996` | **present** | Recent, kept in cache |
| `key_3997` | **present** | |
| `key_3998` | **present** | |
| `key_3999` | **present** | |
| `key_4000` | **present** | Most recent, always retained |

Early keys are evicted first. Recent keys are always retained. The eviction order matches least-recently-used semantics.

---

## Reproduce locally

### Native

```bash
./blink-store serve --tcp 127.0.0.1:8765 --memory-limit 2097152

# In another terminal
python3 scripts/benchmark.py
```

### Container

```bash
podman run -d --name blink-bench \
  --cpus 1 --memory 2g \
  -p 8765:8765 \
  -e BLINK_MEMORY_LIMIT=2097152 \
  ghcr.io/ashokdudhade/blink-store:latest

python3 scripts/benchmark.py
```

Replace `podman` with `docker` if using Docker. The benchmark script lives at [`scripts/benchmark.py`](https://github.com/ashokdudhade/blink-store/blob/main/scripts/benchmark.py).

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
| `SET` | 64 bytes | ~9,500 |
| `SET` | 256 bytes | ~11,400 |
| `SET` | 1 KiB | ~14,500 |
| `GET` | — | ~16,500 |
| `DELETE` | — | ~15,900 |

### Latency

| Operation | avg | p50 | p95 | p99 | max |
|-----------|-----|-----|-----|-----|-----|
| `SET` | 76 us | 47 us | 170 us | 432 us | 11.36 ms |
| `GET` | 64 us | 50 us | 88 us | 171 us | 16.06 ms |

---

## Container (1 CPU, 2 GB memory)

Server runs inside a container built from the project `Dockerfile`, restricted to 1 CPU and 2 GB memory via Podman/Docker resource limits.

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
| `SET` | 64 bytes | ~5,200 |
| `SET` | 256 bytes | ~6,500 |
| `SET` | 1 KiB | ~6,100 |
| `GET` | — | ~6,100 |
| `DELETE` | — | ~6,300 |

### Latency

| Operation | avg | p50 | p95 | p99 | max |
|-----------|-----|-----|-----|-----|-----|
| `SET` | 151 us | 136 us | 217 us | 357 us | 1.81 ms |
| `GET` | 153 us | 136 us | 216 us | 387 us | 1.71 ms |

### Native vs. container comparison

| Metric | Native | Container (1 CPU, 2 GB) | Overhead |
|--------|--------|------------------------|----------|
| SET 256B throughput | 11,400 ops/s | 6,500 ops/s | ~43% slower |
| GET throughput | 16,500 ops/s | 6,100 ops/s | ~63% slower |
| SET p50 latency | 47 us | 136 us | ~2.9x |
| GET p50 latency | 50 us | 136 us | ~2.7x |
| SET p95 latency | 170 us | 217 us | ~1.3x |
| GET p95 latency | 88 us | 216 us | ~2.5x |
| SET p99 latency | 432 us | 357 us | 0.8x (within noise) |
| GET p99 latency | 171 us | 387 us | ~2.3x |

The container overhead comes from two layers: the WSL2 virtual network adapter and the container network namespace. On bare-metal Linux with Docker, expect significantly less overhead.

---

## Memory cap enforcement

Proves that `--memory-limit` is respected and sampled eviction works correctly. Results are identical in both native and container runs.

**Setup:** 2 MiB memory limit, inserting 4,000 keys with 1 KiB values each (~4 MiB of data into a 2 MiB store).

| Metric | Result |
|--------|--------|
| Keys inserted | 4,000 |
| Data attempted | ~4 MiB |
| Memory limit | 2,097,152 bytes (2 MiB) |
| Final usage (native) | 2,097,037 bytes |
| Final usage (container) | 2,097,147 bytes |
| Usage vs. limit | **~99.99%** — right at the boundary, never exceeded |

### Sampled eviction verification

Blink Store uses sampled eviction (similar to Redis). Rather than maintaining a strict LRU list, it samples a fixed number of entries and evicts the least-recently-accessed one. This is probabilistic — older keys are statistically more likely to be evicted, but not guaranteed in exact LRU order.

| Key | Status | Notes |
|-----|--------|-------|
| `memtest_1` | evicted | Early key, high eviction probability |
| `memtest_2` | evicted | |
| `memtest_3` | evicted or present | Probabilistic — may survive sampling |
| `memtest_10` | evicted or present | |
| `memtest_50` | evicted | |
| `memtest_100` | evicted or present | |
| `memtest_3996`–`memtest_4000` | **all present** | Recent keys always retained |

The benchmark requires at least half of the early keys to be evicted and all five most recent keys to be present. Both native and container runs passed with 4/6 early keys evicted.

---

## Reproduce locally

### Native

```bash
cargo build --release
./target/release/blink-store serve --tcp 127.0.0.1:8765 --memory-limit 2097152

# In another terminal
python3 scripts/benchmark.py
```

### Container

```bash
podman build -t blink-store:bench .

podman run -d --name blink-bench \
  --cpus 1 --memory 2g \
  -p 8765:8765 \
  -e BLINK_MEMORY_LIMIT=2097152 \
  blink-store:bench

python3 scripts/benchmark.py

podman stop blink-bench && podman rm blink-bench
```

Replace `podman` with `docker` if using Docker. The benchmark script lives at [`scripts/benchmark.py`](https://github.com/ashokdudhade/blink-store/blob/main/scripts/benchmark.py).

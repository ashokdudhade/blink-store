---
sidebar_position: 6
title: Benchmarks
---

# Benchmarks

Measured on a single persistent TCP connection, sequential operations, 10,000 operations per test. Server configured with `--memory-limit 2097152` (2 MiB), restricted to 1 CPU and 2 GB memory.

:::info Test environment
| | |
|:--|:--|
| **CPU** | Intel Core 7 150U — 6 cores / 12 threads |
| **Memory** | 7.6 GiB (WSL2 allocation) |
| **OS** | Ubuntu 24.04 LTS on WSL2 |
| **Kernel** | 6.6.87.2-microsoft-standard-WSL2 |
| **Arch** | x86_64 |
| **Image** | `ghcr.io/ashokdudhade/blink-store:latest` (v0.1.4) |

Results will vary by hardware and network stack.
:::

---

## Throughput

| Operation | Value size | ops/sec |
|-----------|-----------|---------|
| `SET` | 64 bytes | ~5,370 |
| `SET` | 256 bytes | ~4,780 |
| `SET` | 1 KiB | ~8,140 |
| `GET` | — | ~7,200 |
| `DELETE` | — | ~8,000 |

## Latency

| Operation | avg | p50 | p95 | p99 | max |
|-----------|-----|-----|-----|-----|-----|
| `SET` | 158 us | 145 us | 264 us | 451 us | 10.64 ms |
| `GET` | 125 us | 111 us | 200 us | 312 us | 1.19 ms |

---

## Memory cap enforcement

Proves that `--memory-limit` is respected and sampled eviction works correctly.

**Setup:** 2 MiB memory limit, inserting 4,000 keys with 1 KiB values each (~4 MiB of data into a 2 MiB store).

| Metric | Result |
|--------|--------|
| Keys inserted | 4,000 |
| Data attempted | ~4 MiB |
| Memory limit | 2,097,152 bytes (2 MiB) |
| Final usage | 2,097,046 bytes |
| Usage vs. limit | **~99.99%** — right at the boundary, never exceeded |

### Sampled eviction verification

Blink Store uses sampled eviction (similar to Redis). Rather than maintaining a strict LRU list, it samples a fixed number of entries and evicts the least-recently-accessed one. This is probabilistic — older keys are statistically more likely to be evicted, but not guaranteed in exact LRU order.

| Key | Status | Notes |
|-----|--------|-------|
| `memtest_1` | evicted or present | Probabilistic — depends on sampling |
| `memtest_2` | evicted or present | |
| `memtest_3` | evicted or present | |
| `memtest_10` | evicted or present | |
| `memtest_50` | evicted or present | |
| `memtest_100` | evicted or present | |
| `memtest_3996`–`memtest_4000` | **all present** | Recent keys always retained |

The benchmark requires at least half of the early keys to be evicted and all five most recent keys to be present. The run passed with 4/6 early keys evicted.

---

## Reproduce

```bash
podman run -d --name blink-bench \
  --cpus 1 --memory 2g \
  -p 8765:8765 \
  -e BLINK_MEMORY_LIMIT=2097152 \
  ghcr.io/ashokdudhade/blink-store:latest

python3 scripts/benchmark.py

podman stop blink-bench && podman rm blink-bench
```

Replace `podman` with `docker` if using Docker. The benchmark script lives at [`scripts/benchmark.py`](https://github.com/ashokdudhade/blink-store/blob/main/scripts/benchmark.py).

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

Results will vary by hardware and network stack. WSL2 adds a small overhead compared to bare-metal Linux due to the Hyper-V virtual network adapter.
:::

---

## Throughput

| Operation | Value size | ops/sec |
|-----------|-----------|---------|
| `SET` | 64 bytes | ~8,950 |
| `SET` | 256 bytes | ~9,600 |
| `SET` | 1 KiB | ~8,800 |
| `GET` | — | ~11,100 |
| `DELETE` | — | ~9,700 |

Throughput is consistent across value sizes from 64 bytes to 1 KiB. `GET` is the fastest operation since it avoids the LRU bookkeeping write path.

---

## Latency

Measured per-operation on a single connection with 10,000 sequential samples (256-byte values).

| Operation | avg | p50 | p95 | p99 | max |
|-----------|-----|-----|-----|-----|-----|
| `SET` | 106 us | 85 us | 207 us | 347 us | 5.97 ms |
| `GET` | 91 us | 81 us | 151 us | 231 us | 906 us |

Median latency is under **100 microseconds** for both reads and writes. The p99 stays under 350 us, with occasional outliers at the tail from OS scheduling and network jitter.

---

## Memory cap enforcement

Proves that `--memory-limit` is respected and LRU eviction works correctly.

**Setup:** 2 MiB memory limit, inserting 4,000 keys with 1 KiB values each (~4 MiB of data into a 2 MiB store).

| Metric | Result |
|--------|--------|
| Keys inserted | 4,000 |
| Data attempted | ~4 MiB |
| Memory limit | 2,097,152 bytes (2 MiB) |
| Final usage | 2,096,715 bytes (2,048 KiB) |
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

Run the benchmark yourself:

```bash
# Start the server with a 2 MiB limit
./blink-store serve --tcp 127.0.0.1:8765 --memory-limit 2097152

# In another terminal
python3 scripts/benchmark.py
```

The benchmark script lives at [`scripts/benchmark.py`](https://github.com/ashokdudhade/blink-store/blob/main/scripts/benchmark.py) in the repository.

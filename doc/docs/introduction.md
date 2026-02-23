---
sidebar_position: 1
title: Introduction
slug: /introduction
---

# Blink Store

> Blazing-fast in-memory key-value store. Single binary. Any language.

Blink Store is a blazing-fast in-memory key-value store built in Rust. It ships as a single binary with no runtime dependencies — download it, run it, and connect from any language over TCP. Sub-50 µs median latency. 16K+ ops/sec on a single connection.

It is designed for teams that need a **simple, fast local cache** without the overhead of Redis, Memcached, or an external service. Install in seconds, configure with flags, integrate with a few lines of socket code.

---

## At a glance

| | |
|:---|:---|
| **Single binary, zero config** | One `curl` command to install. One flag to start. No config files, no daemon setup, no package manager. |
| **Works with every language** | Plain-text TCP protocol. If your language can open a socket and send a line of text, it can use Blink Store. Clients shown for Python, Node.js, Go, Bash, and Rust. |
| **Smart memory management** | Set a byte limit with `--memory-limit`. Blink Store tracks the size of every key and value and uses sampled eviction (similar to Redis) to reclaim space when the limit is reached. |
| **Written in Rust** | No `unsafe` code in the library. Error handling with `Result` throughout — no panics in production paths. Structured logging via `tracing`. |
| **Concurrent by design** | Storage backed by `DashMap` (lock-free concurrent hash map). Async I/O with Tokio. Each client connection is served on its own lightweight task. |
| **Cross-platform** | Pre-built release binaries for Linux (x86_64, aarch64), macOS (x86_64, arm64), and Windows (x86_64). Docker images for `linux/amd64` and `linux/arm64`. |
| **TCP and Unix sockets** | Listen on `--tcp`, `--unix`, or both at the same time. |

---

## Quick start

Install the latest release and start the server (no Git clone, no Rust toolchain):

```bash
curl -sSLf https://raw.githubusercontent.com/ashokdudhade/blink-store/main/scripts/install-from-github.sh \
  | bash -s -- latest ./bin

./bin/blink-store serve --tcp 127.0.0.1:8765
```

Talk to it from any terminal:

```bash
echo "SET user alice" | nc 127.0.0.1 8765    # → OK
echo "GET user"       | nc 127.0.0.1 8765    # → VALUE YWxpY2U=
echo "USAGE"          | nc 127.0.0.1 8765    # → USAGE 9
echo "DELETE user"    | nc 127.0.0.1 8765    # → OK
```

:::tip
`VALUE` responses are base64-encoded. Decode: `echo YWxpY2U= | base64 -d` → `alice`.
:::

---

## Protocol

Five commands. That's the entire API.

| Command | Example | Response |
|---------|---------|----------|
| `SET key value` | `SET user alice` | `OK` |
| `GET key` | `GET user` | `VALUE YWxpY2U=` |
| `DELETE key` | `DELETE user` | `OK` or `NOT_FOUND` |
| `USAGE` | `USAGE` | `USAGE 9` |
| `QUIT` | `QUIT` | *(connection closed)* |

Full specification: [Protocol Reference](protocol).

---

## Architecture

```text
                   ┌──────────────────────────────────┐
 TCP / Unix socket │          Blink Store              │
 ────────────────→ │                                    │
                   │  ┌──────────┐  ┌────────────────┐ │
  SET / GET / DEL  │  │ DashMap  │ ←│    Sampled     │ │
 ←──────────────── │  │ (store)  │  │   Eviction     │ │
                   │  └──────────┘  └────────────────┘ │
                   │       ↑                            │
                   │  AtomicU64 (size tracking)         │
                   └──────────────────────────────────┘
```

- **DashMap** — lock-free concurrent hash map. Multiple connections read and write without blocking each other.
- **Sampled eviction** — each entry stores a monotonic access counter. When the memory limit is exceeded, a sample of entries is inspected and the least-recently-accessed one is evicted. This is the same strategy Redis uses — probabilistic, low-overhead, and effective.
- **AtomicU64** — tracks total stored bytes (key length + value length) with atomic operations. No locks.
- **Tokio** — async runtime. Each connection is a lightweight task, not a thread.

---

## Use cases

- **Local development cache** — Drop-in replacement for Redis/Memcached during development. No Docker, no config.
- **Sidecar cache** — Run alongside your application for low-latency caching without network hops to an external service.
- **CI/CD ephemeral store** — Spin up a cache in your test pipeline with a single command. Tear it down when done.
- **Prototyping** — Add caching to any project in minutes. The protocol is simple enough to implement inline.
- **Multi-language environments** — Share cached data between services written in different languages over TCP.

---

## When to use something else

Blink Store is deliberately simple. Reach for Redis, Valkey, or Memcached when you need:

- **Persistence** — Blink Store is ephemeral. Data is lost when the process stops.
- **Clustering / replication** — Blink Store is single-node.
- **Rich data structures** — Lists, sets, sorted sets, streams, pub/sub.
- **Access control** — Blink Store has no authentication or authorization.

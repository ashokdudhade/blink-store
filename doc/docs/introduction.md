---
sidebar_position: 1
title: Introduction
slug: /introduction
---

# Blink-Store

> In-memory key-value store. Single binary. Any language.

Blink-Store is an in-memory key-value store built in Rust. It ships as a single binary with no runtime dependencies — download it, run it, and connect from any language over TCP.

It is designed for teams that need a **simple, fast local cache** without the overhead of Redis, Memcached, or an external service. Install in seconds, configure with flags, integrate with a few lines of socket code.

---

## At a glance

| | |
|:---|:---|
| **Single binary, zero config** | One `curl` command to install. One flag to start. No config files, no daemon setup, no package manager. |
| **Works with every language** | Plain-text TCP protocol. If your language can open a socket and send a line of text, it can use Blink-Store. Clients shown for Python, Node.js, Go, Bash, and Rust. |
| **Built-in memory management** | Set a byte limit with `--memory-limit`. Blink-Store tracks the size of every key and value and automatically evicts least-recently-used entries when the limit is reached. |
| **Written in Rust** | No `unsafe` code in the library. Error handling with `Result` throughout — no panics in production paths. Structured logging via `tracing`. |
| **Concurrent by design** | Storage backed by `DashMap` (lock-free concurrent hash map). Async I/O with Tokio. Each client connection is served on its own task. |
| **Cross-platform** | Pre-built release binaries for Linux (x86_64, aarch64), macOS (x86_64, arm64), and Windows (x86_64). Also runs in Docker. |
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
| `DELETE key` | `DELETE user` | `OK` |
| `USAGE` | `USAGE` | `USAGE 9` |
| `QUIT` | `QUIT` | *(connection closed)* |

Full specification: [Protocol Reference](protocol).

---

## How it works

```text
                      ┌─────────────────────────────┐
  TCP / Unix socket   │         Blink-Store          │
  ─────────────────→  │                               │
                      │  ┌─────────┐   ┌───────────┐ │
   SET / GET / DELETE  │  │ DashMap │ ← │ LRU Cache │ │
  ←─────────────────  │  │ (store) │   │ (eviction)│ │
                      │  └─────────┘   └───────────┘ │
                      │       ↑                       │
                      │  AtomicU64 (size tracking)    │
                      └─────────────────────────────┘
```

- **DashMap** — concurrent hash map for key-value storage. Lock-free reads.
- **LRU cache** — tracks access order. When `--memory-limit` is exceeded, the least-recently-used key is evicted.
- **AtomicU64** — tracks total stored bytes (key length + value length) with atomic operations.
- **Tokio** — async runtime. Each connection is a lightweight task.

---

## Use cases

- **Local development cache** — Drop-in replacement for Redis/Memcached during development. No Docker, no config.
- **Sidecar cache** — Run alongside your application for low-latency caching without network hops to an external service.
- **CI/CD ephemeral store** — Spin up a cache in your test pipeline with a single command. Tear it down when done.
- **Prototyping** — Add caching to any project in minutes. The protocol is simple enough to implement inline.

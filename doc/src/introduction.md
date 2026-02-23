# Introduction

**Blink-Store** is a small, fast in-memory key-value store with **LRU eviction** and a configurable **memory cap**. It uses a simple line-based protocol over TCP or Unix sockets, so you can use it from any language.

## Why Blink-Store

- **Embeddable or standalone** — Run as a sidecar, local cache, or network service.
- **Memory-safe** — No unsafe code; strict size limits.
- **Protocol-first** — One text protocol for every client.
- **Multi-language** — Examples in Rust, Python, Node.js, Go, and Shell.

## Features

| Feature | Description |
|--------|-------------|
| BlinkStorage trait | Abstraction for in-memory and future backends. |
| LRU eviction | Least-recently-used keys evicted when over the byte limit. |
| Size tracking | Stored size in bytes (key + value). |
| TCP and Unix | Server listens on TCP and/or Unix socket. |
| Logging | Structured tracing; optional log retention. |

## Quick start

```bash
./scripts/install-from-github.sh ./bin
./bin/blink-store serve --tcp 127.0.0.1 8765
```

Then connect with any client. The rest of this book covers installation, protocol, language guides, and deployment.

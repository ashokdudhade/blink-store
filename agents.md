# Blink-Store Development Plan

## Phase 1: Core Architecture & Abstractions
- [x] Define the `BlinkStorage` trait (Abstraction layer for In-Memory/Redis).
- [x] Implement the `LruCache` logic with memory-cap enforcement.
- [x] Setup `DashMap` or `Mutex<HashMap>` for thread-safe access.

## Phase 2: Memory Management
- [x] Implement size tracking for stored values.
- [x] Eviction policy: Remove least recently used items when `current_usage + new_item > limit`.

## Phase 3: Logging & Retention
- [x] Integrate `tracing` and `tracing-appender`.
- [x] Implement a background worker to prune log files based on `RETENTION_MINUTES`.

## Phase 4: Containerization & CLI
- [x] Create `Dockerfile` (multi-stage build for small footprint).
- [x] Add CLI flags for memory limit and retention policy.

## Phase 5: Multi-Platform Distribution
- [x] **Cross-Compilation Setup**:
    - Use `cross` or `cargo-zigbuild` for Linux (musl), Windows (msvc), and macOS (darwin).
- [x] **Release Pipeline**:
    - Configure GitHub Actions to automate binary uploads for `x86_64` and `aarch64`.
- [x] **Interface Layer**:
    - Implement a lightweight TCP or Unix Domain Socket listener so any language can connect.
- [x] **Example Clients**:
    - Create a standard protocol (e.g., Simple Text or Protobuf) for client interactions.
    - Add all popular programming language examples (Rust, Python, Node.js, Go, Shell in `examples/clients/`).
    - Add examples in such way that it shows usage in backend application (HTTP backends in Rust, Python, Node, Go that use Blink-Store as cache: `backend_http`, `backend_app.py`, `backend_app.js`, `backend_app.go`). 
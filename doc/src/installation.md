# Installation

## From GitHub release

Download the binary for your OS/arch (no Rust required). Default is the **latest** release; pass a version tag to pin (e.g. `v0.1.0`).

```bash
./scripts/install-from-github.sh ./bin
# or pin version: ./scripts/install-from-github.sh v0.1.0 ./bin
./bin/blink-store serve --tcp 127.0.0.1 8765
```

Or without cloning: fetch [install-from-github.sh](https://github.com/ashokdudhade/blink-store/blob/main/scripts/install-from-github.sh) and run `./install-from-github.sh ./bin` (or `./install-from-github.sh v0.1.0 ./bin` to fix version). Supported: Linux (x86_64, aarch64), macOS (x86_64, arm64), Windows (x86_64).

## From source (Rust)

Install Rust from [rustup](https://rustup.rs), then:

```bash
git clone https://github.com/ashokdudhade/blink-store
cd blink-store
cargo build --release
```

Binary: `target/release/blink-store`. Run `./target/release/blink-store --help`.

## Local distribution

```bash
./scripts/build-dist.sh
```

This creates `dist/blink-store`, `dist/blink_client`, and `dist/backend_http`. Then:

```bash
./dist/blink-store serve --tcp 127.0.0.1 8765
```

## Docker

```bash
docker compose up -d
```

Server listens on port 8765. Or build and run manually:

```bash
docker build -t blink-store .
docker run -p 8765:8765 blink-store serve --tcp 0.0.0.0:8765
```

## Verify

Start the server, then use any client (e.g. `python examples/clients/python/blink_client.py` or `cargo run --example blink_client -- --tcp 127.0.0.1:8765`). Server: `./bin/blink-store serve --tcp 127.0.0.1 8765`. Try: `SET hello world`, `GET hello`, `QUIT`.

# Installation

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

Start the server, then:

```bash
./dist/blink_client --tcp 127.0.0.1:8765
```

Try: `SET hello world`, `GET hello`, `QUIT`.

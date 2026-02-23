# Example: Using a release from GitHub

This example runs **Blink-Store** using a binary from [GitHub Releases](https://github.com/ashokdudhade/blink-store/releases). No Rust toolchain or repo build required.

- Use **latest** (default) to always get the newest release.
- Use a **version tag** (e.g. `v0.1.0`) to pin and get the same binary every time.

## 1. Download the binary

From a clone of the repo (default: latest, install into `./bin`):

```bash
./scripts/install-from-github.sh ./bin
```

Pin to a specific version:

```bash
./scripts/install-from-github.sh v0.1.0 ./bin
```

Or without cloning (Linux/macOS with `curl`):

```bash
curl -sSLf https://raw.githubusercontent.com/ashokdudhade/blink-store/main/scripts/install-from-github.sh -o install-from-github.sh
chmod +x install-from-github.sh
./install-from-github.sh ./bin
./install-from-github.sh v0.1.0 ./bin
```

This installs `./bin/blink-store` (or `blink-store.exe` on Windows).

## 2. Start the server

```bash
./bin/blink-store serve --tcp 127.0.0.1 8765
```

## 3. Use a client

The GitHub release only includes the `blink-store` server binary. For a client you can:

- Use any of the **language examples** in this repo (Python, Node, Go, Shell) — they talk the same protocol.
- Or build the Rust client from source: `cargo run --example blink_client -- --tcp 127.0.0.1:8765`.

Example with the Python client (from repo root):

```bash
python examples/clients/python/blink_client.py
# Then: SET hello world, GET hello, QUIT
```

## Tags and versions

| Tag / usage | Meaning |
|-------------|---------|
| **latest** | Moving tag updated on each release; always points to the newest release. |
| **v0.1.0**, **v0.2.0**, … | Permanent version tags; use these to pin a specific release. |

Each new release (e.g. pushing `v0.2.0`) creates a GitHub Release and updates the **latest** tag. Version tags are never overwritten so consumers can fix a version when required.

## Asset names

| Platform   | Asset name |
|-----------|------------|
| Linux x86_64 | `blink-store-x86_64-unknown-linux-gnu` |
| Linux aarch64 | `blink-store-aarch64-unknown-linux-gnu` |
| macOS x86_64 | `blink-store-x86_64-apple-darwin` |
| macOS arm64 | `blink-store-aarch64-apple-darwin` |
| Windows x86_64 | `blink-store-x86_64-pc-windows-msvc.exe` |

To create a release, push a version tag (e.g. `git tag v0.1.0 && git push origin v0.1.0`). The [Release workflow](.github/workflows/release.yml) builds, publishes assets, and tags the release as **latest**.

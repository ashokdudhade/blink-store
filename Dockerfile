# Multi-stage build for small footprint
FROM rust:1-slim-bookworm AS builder
WORKDIR /app
COPY Cargo.toml Cargo.lock* ./
COPY src ./src
RUN cargo build --release

# Runtime stage: minimal image
FROM debian:bookworm-slim
RUN apt-get update -y && apt-get install -y --no-install-recommends ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/blink-store /usr/local/bin/
ENTRYPOINT ["/usr/local/bin/blink-store"]
CMD ["run", "--memory-limit", "10485760", "--retention-minutes", "60"]

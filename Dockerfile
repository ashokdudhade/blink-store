FROM rust:1-alpine AS builder
RUN apk add --no-cache musl-dev
WORKDIR /app
COPY Cargo.toml Cargo.lock* ./
COPY src ./src
RUN cargo build --release && strip target/release/blink-store

FROM alpine:3
RUN apk add --no-cache tini
COPY --from=builder /app/target/release/blink-store /usr/local/bin/

ENV BLINK_PORT=8765
ENV BLINK_MEMORY_LIMIT=10485760
ENV BLINK_RETENTION_MINUTES=60

EXPOSE ${BLINK_PORT}

ENTRYPOINT ["tini", "--"]
CMD sh -c 'exec blink-store serve \
  --tcp "0.0.0.0:${BLINK_PORT}" \
  --memory-limit "${BLINK_MEMORY_LIMIT}" \
  --retention-minutes "${BLINK_RETENTION_MINUTES}"'

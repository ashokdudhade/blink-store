FROM alpine:3

ARG TARGETARCH
ARG BLINK_VERSION=latest

RUN apk add --no-cache tini curl \
    && case "${TARGETARCH}" in \
         amd64) TRIPLE="x86_64-unknown-linux-musl" ;; \
         arm64) TRIPLE="aarch64-unknown-linux-musl" ;; \
         *) echo "unsupported arch: ${TARGETARCH}" && exit 1 ;; \
       esac \
    && curl -sSLf -o /usr/local/bin/blink-store \
       "https://github.com/ashokdudhade/blink-store/releases/download/${BLINK_VERSION}/blink-store-${TRIPLE}" \
    && chmod +x /usr/local/bin/blink-store \
    && apk del curl

ENV BLINK_PORT=8765
ENV BLINK_MEMORY_LIMIT=10485760
ENV BLINK_RETENTION_MINUTES=60

EXPOSE ${BLINK_PORT}

ENTRYPOINT ["tini", "--"]
CMD sh -c 'exec blink-store serve \
  --tcp "0.0.0.0:${BLINK_PORT}" \
  --memory-limit "${BLINK_MEMORY_LIMIT}" \
  --retention-minutes "${BLINK_RETENTION_MINUTES}"'

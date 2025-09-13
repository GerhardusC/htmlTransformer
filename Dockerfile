# Stage 1: Build
FROM rust AS builder

WORKDIR /

# Copy everything at once (no caching)
COPY . .

RUN cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
 && rm -rf /var/lib/apt/lists/*

WORKDIR /

COPY --from=builder /target/release/rocketseed-interview /app

EXPOSE 3000
CMD ["/app"]

# syntax=docker/dockerfile:1.6

############################
# Build stage
############################
FROM rust:1.83-bookworm AS builder
WORKDIR /app

RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config \
    ca-certificates \
    libssl-dev \
 && rm -rf /var/lib/apt/lists/*

COPY Cargo.toml Cargo.lock ./
RUN mkdir -p src && echo "fn main(){}" > src/main.rs

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    cargo build --release

COPY . .
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    cargo build --release

############################
# Runtime stage
############################
FROM debian:bookworm-slim AS runtime
WORKDIR /app

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    libssl3 \
 && rm -rf /var/lib/apt/lists/*

RUN useradd -m -u 10001 appuser

COPY --from=builder /app/target/release/opentelemetry-demo-app /usr/local/bin/opentelemetry-demo-app
COPY config.yml /app/config.yml

USER appuser
ENV RUST_LOG=info

ENTRYPOINT ["/usr/local/bin/opentelemetry-demo-app"]
CMD ["-f", "/app/config.yml"]
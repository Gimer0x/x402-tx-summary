# syntax=docker/dockerfile:1.7
FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR /app

# 1) Compute dependency recipe
FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# 2) Build dependencies + app
FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --release --locked --bin axum-x402

# 3) Minimal runtime image
FROM debian:bookworm-slim AS runtime
RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=builder /app/target/release/axum-x402 /usr/local/bin/axum-x402

EXPOSE 8080
ENTRYPOINT ["/usr/local/bin/axum-x402"]
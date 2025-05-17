# ---- Build Stage ----
FROM rust:latest AS builder
WORKDIR /usr/src/hogger

COPY Cargo.toml Cargo.lock* ./
COPY src ./src

RUN cargo build --release

# ---- Runtime Stage ----
FROM debian:sid-slim

RUN apt-get update && apt-get install -y libssl3 ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/hogger

# Correct paths
COPY --from=builder /usr/src/hogger/target/release/hogger ./hogger
COPY --from=builder /usr/src/hogger/target/release/hogger-worker ./hogger-worker

# Expose the default port for the API
EXPOSE 3000

# Default command: API
CMD ["./hogger"]

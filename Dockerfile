# ---- Build Stage ----
FROM rust:latest AS builder
WORKDIR /usr/src/hogger

# Copy full source code and build
COPY Cargo.toml .
COPY Cargo.lock . 
COPY src ./src
RUN cargo build --release

# ---- Runtime Stage ----
FROM debian:sid-slim

RUN apt-get update && apt-get install -y libssl3 ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/hogger

COPY --from=builder /usr/src/hogger/target/release/hogger ./hogger
COPY --from=builder /usr/src/hogger/target/release/hogger-worker ./hogger-worker
COPY --from=builder /usr/src/hogger/target/release/hogger-bulk-worker ./hogger-bulk-worker

EXPOSE 3000

CMD ["./hogger"]

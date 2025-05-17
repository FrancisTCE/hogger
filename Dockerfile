# Start from the official Rust image with latest stable toolchain
FROM rust:latest AS builder

# Create app directory inside container
WORKDIR /usr/src/hogger

# Copy Cargo.toml and Cargo.lock (if present) first to cache dependencies
COPY Cargo.toml Cargo.lock* ./

# Copy source code
COPY src ./src

# Build release binary (using the 2024 edition works with latest Rust)
RUN cargo build --release

# ---- Runtime Stage ----
FROM debian:bookworm-slim

# Install dependencies required to run the binary (e.g. SSL libs)
RUN apt-get update && apt-get install -y libssl3 ca-certificates && rm -rf /var/lib/apt/lists/*

# Create app directory
WORKDIR /usr/src/hogger

# Copy the compiled binary from the builder stage
COPY --from=builder /usr/src/hogger/target/release/hogger .

# Expose the port your app uses (change if needed)
EXPOSE 3000

# Run the binary
CMD ["./hogger"]

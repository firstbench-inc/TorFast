# Step 1: Build the application
FROM rust:latest AS builder

# Create app directory
WORKDIR /app

# Copy the entire project into the container
COPY . .

# Build the Rust application
RUN cargo build --release

# Step 2: Create a minimal image with the built binary
FROM debian:bookworm-slim

# Install the necessary dependencies for running the binary
RUN apt-get update && apt-get install -y \
    libssl-dev \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*\
    && apt-get update && apt install -y openssl

# Create a non-root user and switch to it
# Copy the built binary from the builder stage
COPY --from=builder /app/target/release/crawle-rs /usr/local/bin/crawle-rs

# Copy any other necessary files (e.g., configuration files)
# COPY config /path/to/config

# Set the entrypoint to the built binary
ENTRYPOINT ["crawle-rs"]
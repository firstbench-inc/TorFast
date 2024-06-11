# Step 1: Build the application
FROM rust:latest AS builder

# Create app directory
WORKDIR /app

# Copy the entire project into the container
COPY . .

# Build the Rust application
RUN cargo build --release

# Step 2: Create a minimal image with the built binary
FROM debian:bullseye-slim

# Install dependencies
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    openssl \
    tor \
    openjdk-11-jre-headless \
    && rm -rf /var/lib/apt/lists/*

# Install Elasticsearch
RUN apt-get update && \
    apt-get install -y wget gnupg && \
    wget -qO - https://artifacts.elastic.co/GPG-KEY-elasticsearch | apt-key add - && \
    echo "deb https://artifacts.elastic.co/packages/oss-7.x/apt stable main" | tee -a /etc/apt/sources.list.d/elastic-7.x.list && \
    apt-get update && \
    apt-get install -y elasticsearch-oss && \
    apt-get clean

# Configure Elasticsearch
RUN echo "network.host: 0.0.0.0" >> /etc/elasticsearch/elasticsearch.yml \
    && echo "http.port: 9200" >> /etc/elasticsearch/elasticsearch.yml \
    && echo "xpack.security.enabled: false"  >> /etc/elasticsearch/elasticsearch.yml \
    && echo "xpack.security.http.ssl.enabled: false"  >> /etc/elasticsearch/elasticsearch.yml \
    && echo "xpack.security.transport.ssl.enabled: false"  >> /etc/elasticsearch/elasticsearch.yml

# Start Tor
RUN tor --RunAsDaemon 1

# Additional network checks and troubleshooting
RUN apt-get update && \
    apt-get install -y netcat curl && \
    apt-get clean

# Check Tor connectivity
RUN nc -z -v localhost 9050 || exit 1

# Check if Tor can access the internet
RUN curl --socks5-hostname localhost:9050 https://check.torproject.org || exit 1

# Copy the built binary from the builder stage
COPY --from=builder /app/target/release/crawle-rs /usr/local/bin/crawle-rs

# Set the entrypoint to the built binary
ENTRYPOINT ["crawle-rs"]

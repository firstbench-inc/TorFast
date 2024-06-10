
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
    wget \
    gnupg2 \
    lsb-release \
    && rm -rf /var/lib/apt/lists/*
RUN apt-get install apt-transport-https
# Install Elasticsearch
RUN wget -qO - https://artifacts.elastic.co/GPG-KEY-elasticsearch | gpg --dearmor -o /usr/share/keyrings/elasticsearch-keyring.gpg \
    && echo "deb [signed-by=/usr/share/keyrings/elasticsearch-keyring.gpg] https://artifacts.elastic.co/packages/8.x/apt stable main" | tee /etc/apt/sources.list.d/elastic-8.x.list \
    && apt-get update 
RUN apt-get install -y elasticsearch
# Install Tor
RUN apt-get update && apt-get install -y tor

# Configure Elasticsearch
RUN echo "network.host: 0.0.0.0" >> /etc/elasticsearch/elasticsearch.yml \
    && echo "http.port: 9200" >> /etc/elasticsearch/elasticsearch.yml \
    && echo "xpack.security.enabled: false"  >> /etc/elasticsearch/elasticsearch.yml \
    && echo "xpack.security.http.ssl.enabled: false"  >> /etc/elasticsearch/elasticsearch.yml \
    && echo "xpack.security.transport.ssl.enabled: false"  >> /etc/elasticsearch/elasticsearch.yml

# Start Elasticsearch
#RUN /bin/systemctl daemon-reload
#RUN /bin/systemctl enable elasticsearch.service
#RUN systemctl start elasticsearch.service

# Create a non-root user and switch to it
RUN useradd -m appuser
USER appuser

# Copy the built binary from the builder stage
COPY --from=builder /app/target/release/crawle-rs /usr/local/bin/crawle-rs

# Set the entrypoint to the built binary
ENTRYPOINT ["crawle-rs"]


# Set the entrypoint to the built binary
ENTRYPOINT ["crawle-rs"]



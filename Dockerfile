# Step 1: Build the application
FROM rust:latest AS builder

# Create app directory
WORKDIR /app

# Copy the entire project into the container
COPY . .

# Build the Rust application
RUN cargo build --release

# Step 2: Create a minimal image with the built binary
# FROM archlinux:latest
FROM greyltc/archlinux-aur:paru

RUN useradd -m appuser
USER appuser

# Install the necessary dependencies for running the binary
RUN sudo pacman -Sy --noconfirm && \
    sudo pacman -S --noconfirm openssl tor && \
    sudo pacman -Scc --noconfirm

# Install Elasticsearch
RUN paru -Sy --noconfirm && \
    paru -S --noconfirm elasticsearch && \
    paru -Scc --noconfirm

# Configure Elasticsearch
RUN echo "network.host: 0.0.0.0" >> /etc/elasticsearch/elasticsearch.yml \
    && echo "http.port: 9200" >> /etc/elasticsearch/elasticsearch.yml \
    && echo "xpack.security.enabled: false"  >> /etc/elasticsearch/elasticsearch.yml \
    && echo "xpack.security.http.ssl.enabled: false"  >> /etc/elasticsearch/elasticsearch.yml\
    && echo "xpack.security.transport.ssl.enabled: false"  >> /etc/elasticsearch/elasticsearch.yml


# Start Elasticsearch
RUN systemctl enable elasticsearch.service && \
    systemctl start elasticsearch.service

# Create a non-root user and switch to it


# Copy the built binary from the builder stage
COPY --from=builder /app/target/release/crawle-rs /usr/local/bin/crawle-rs

# Set the entrypoint to the built binary
ENTRYPOINT ["crawle-rs"]


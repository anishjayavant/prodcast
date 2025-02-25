FROM rust:latest AS builder

# Install dependencies
RUN apt-get update && apt-get install -y \
    libpq-dev \
    pkg-config \
    build-essential \
    curl \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Build the project
WORKDIR /app
# Copy the full project
COPY . .
RUN cargo build --release

# Use a minimal Debian base with GLIBC 2.36
FROM debian:bookworm-slim
# Copy the binary from the builder stage
COPY --from=builder /app/target/release/prodcast /usr/local/bin/prodcast
# Copy the config.yml file
COPY --from=builder /app/config.yml /usr/local/bin/config.yml
# Set permissions
RUN chmod +x /usr/local/bin/prodcast
# Set the entrypoint
ENTRYPOINT ["prodcast"]


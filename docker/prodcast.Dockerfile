FROM rust:latest AS builder

RUN apt update && apt install lld clang -y

ENV SQLX_OFFLINE true

# Build the project
WORKDIR /app
# Copy the full project
COPY . .
RUN cargo build --release

# Use a minimal Debian base with GLIBC 2.36
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    libpq-dev \
    pkg-config \
    build-essential \
    curl \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copy the binary from the builder stage
COPY --from=builder /app/target/release/prodcast /usr/local/bin/prodcast
# Copy the config.yml file
COPY --from=builder /app/config.yml /usr/local/bin/config.yml
# Set the entrypoint
ENTRYPOINT ["prodcast"]


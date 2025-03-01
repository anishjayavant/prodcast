FROM rust:latest AS builder

# Install dependencies
RUN apt-get update && apt-get install -y \
    libpq-dev \
    pkg-config \
    build-essential \
    curl \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Install SQLx CLI
RUN cargo install sqlx-cli --no-default-features --features postgres

# Use a minimal Debian base with GLIBC 2.36
FROM debian:bookworm-slim

# Copy SQLx from builder stage
COPY --from=builder /usr/local/cargo/bin/sqlx /usr/local/bin/sqlx

# Set permissions
RUN chmod +x /usr/local/bin/sqlx

WORKDIR /app

# Copy the migrations directory
COPY migrations migrations

ENTRYPOINT ["sqlx"]

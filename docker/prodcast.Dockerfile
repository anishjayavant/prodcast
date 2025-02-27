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

RUN apt-get update -y \
    && apt-get install -y --no-install-recommends openssl ca-certificates \
    # Clean up
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app
    
# Copy the binary from the builder stage
COPY --from=builder /app/target/release/prodcast /app/prodcast
# Copy the config.yml file
COPY --from=builder /app/config .
# Set the entrypoint
ENTRYPOINT ["/app/prodcast"]


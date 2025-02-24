FROM rust:latest

WORKDIR /app

RUN cargo install sqlx-cli --no-default-features --features postgres

ENTRYPOINT ["sqlx"]

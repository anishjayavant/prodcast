# Prodcast

Prodcast is an email newsletter service implemented in Rust, inspired by [Luca Palmieri's book: Zero to Production in Rust](https://www.zero2prod.com).

## Features

- Health check endpoint
- Handling subscriptions
- Configuration management
- Storage management
- Logging and telemetry (in progress)
- Email delivery (todo)

## Code Structure

The `src` directory is structured into the following modules:

- **main.rs**: Application entry point, initializing and running the server.
- **lib.rs**: Library entry point, re-exporting core functionalities.
- **config/**: Loads application settings and environment variables.
- **models/**: Data model for the app.
- **repository/**: Handles storage concerns  
- **routes/**: Defines the HTTP API.
- **service/**: Business logic handlers.

The `tests` directory is structured accordingly and contains unit and integration tests per module.

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)
- [Docker](https://docs.docker.com/get-docker/) (optional, for containerized deployment)

### Installation

1. Clone the repository:

   ```sh
   git clone https://github.com/anishjayavant/prodcast.git
   cd prodcast
   ```

2. Build the project:

   ```sh
   cargo build --release
   ```

3. Set up the environment variables:

   Copy the `.env` file and configure the settings accordingly.

4. Run `docker/postgres.yml` using Docker Compose to start up a local PostGres instance and the associated migrations with sqlx CLI
    ```sh
    docker-compose -f docker/postgres.yml up --build -d
    ```

5. Run the application:

   ```sh
   cargo run --release       
    [2025-02-27T00:01:19Z INFO  actix_server::builder] starting 2 workers
    [2025-02-27T00:01:19Z INFO  prodcast] Prodcast initialized..
    [2025-02-27T00:01:19Z INFO  actix_server::server] Tokio runtime found; starting in existing Tokio runtime
    [2025-02-27T00:01:19Z INFO  actix_server::server] starting service: "actix-web-service-127.0.0.1:8000", workers: 2, listening on: 127.0.0.1:8000
   ```

## Usage

- Use the `/healthz` `GET` endpoint to verify the service is running
    ```sh
    curl -v http://localhost:8000/healthz    
    ```
- Use the `/subscriptions` `POST` endpoint to manage subscribers.
    ```sh
    curl -d 'name=bob%20marley&email=bobmarley@protonmail.com' http://localhost:8000/subscriptions
    ```

## Testing

Run tests using:

```sh
cargo test --all-features
```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.


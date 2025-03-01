//! main.rs
use prodcast::config::app::get_configuration;
use prodcast::run;
use prodcast::telemetry::tracing::{get_subscriber, init_subscriber};
use std::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Initialize tracing
    let subscriber = get_subscriber("prodcast".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    // log current working directory
    tracing::warn!(
        "Current working directory: {:?}",
        std::env::current_dir().unwrap()
    );
    // Read configuration
    let configuration = get_configuration().expect("Failed to read configuration.");
    // log the merged configuration settings
    tracing::info!("Prodcast DB host {}", configuration.database.host);
    tracing::info!("Prodcast DB port {}", configuration.database.port);
    let address = format!(
        "{}:{}",
        configuration.application.host, configuration.application.port
    );
    // Bind the address
    let listener = TcpListener::bind(address)?;
    run(listener, configuration).await?.await
}

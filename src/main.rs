//! main.rs
use env_logger::Env;
use prodcast::config::app::get_configuration;
use prodcast::run;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Initialize logger
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    // log current working directory
    log::warn!(
        "Current working directory: {:?}",
        std::env::current_dir().unwrap()
    );
    // Read configuration
    let configuration = get_configuration().expect("Failed to read configuration.");
    let address = format!(
        "{}:{}",
        configuration.application.host, configuration.application.port
    );
    // Bind the address
    let listener = TcpListener::bind(address)?;
    run(listener, configuration).await?.await
}

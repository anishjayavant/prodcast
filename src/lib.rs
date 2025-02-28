//! lib.rs  
//! Declare all of the sub-modules here so Rust can recognize them as part of the crate.
pub mod config;
pub mod models;
pub mod repository;
pub mod routes;
pub mod service;
pub mod telemetry;
use std::net::TcpListener;
use std::time::Duration;

use crate::repository::newsletter::NewsletterPostGresRepository;
use crate::service::newsletter::NewsletterAppService;
use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use config::app::Settings;
use routes::api::greet;
use routes::api::healthz;
use routes::api::subscribe;
use secrecy::ExposeSecret;
use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;
use tracing_actix_web::TracingLogger;

/// Run the server
pub async fn run(listener: TcpListener, configuration: Settings) -> Result<Server, std::io::Error> {
    // get the connection string
    let connection_string = configuration.database.connection_string();
    // create a connection pool with options
    let connection_pool = PgPoolOptions::new()
        .acquire_timeout(Duration::from_secs(
            configuration.database.connect_timeout_secs,
        ))
        .connect_lazy(connection_string.expose_secret())
        .expect("Failed to create connection pool.");

    // create the newsletter repository
    let newsletter_repository = NewsletterPostGresRepository::new(connection_pool);
    // create the newsletter app service
    let newsletter_app_service = Arc::new(NewsletterAppService::new(newsletter_repository));

    let server = HttpServer::new(move || {
        App::new()
            // enable logger
            .wrap(TracingLogger::default())
            .app_data(web::Data::from(newsletter_app_service.clone()))
            .route("/", web::get().to(greet))
            .route("/healthz", web::get().to(healthz))
            .route("/subscriptions", web::post().to(subscribe))
    })
    .listen(listener)?
    .run();
    // return the server
    tracing::info!("Prodcast initialized..");
    Ok(server)
}

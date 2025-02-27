//! lib.rs  
//! Declare all of the sub-modules here so Rust can recognize them as part of the crate.
pub mod config;
pub mod models;
pub mod repository;
pub mod routes;
pub mod service;
use std::net::TcpListener;

use crate::repository::newsletter::NewsletterPostGresRepository;
use crate::service::newsletter::NewsletterAppService;
use actix_web::dev::Server;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use config::app::Settings;
use routes::api::greet;
use routes::api::healthz;
use routes::api::subscribe;
use std::sync::Arc;

/// Run the server
pub async fn run(listener: TcpListener, configuration: Settings) -> Result<Server, std::io::Error> {
    // get the connection string
    let connection_string = configuration.database.connection_string();
    // create a connection pool
    let connection_pool =
        sqlx::PgPool::connect_lazy(&connection_string).expect("Failed to create connection pool.");

    // create the newsletter repository
    let newsletter_repository = NewsletterPostGresRepository::new(connection_pool);
    // create the newsletter app service
    let newsletter_app_service = Arc::new(NewsletterAppService::new(newsletter_repository));

    let server = HttpServer::new(move || {
        App::new()
            // enable logger
            .wrap(Logger::default())
            .app_data(web::Data::from(newsletter_app_service.clone()))
            .route("/", web::get().to(greet))
            .route("/healthz", web::get().to(healthz))
            .route("/subscriptions", web::post().to(subscribe))
    })
    .listen(listener)?
    .run();
    // return the server
    log::info!("Prodcast initialized..");
    Ok(server)
}

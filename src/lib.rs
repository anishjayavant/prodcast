//! lib.rs  
//! Declare all of the sub-modules here so Rust can recognize them as part of the crate.
pub mod config;
pub mod models;
pub mod repository;
pub mod routes;
pub mod service;
use std::net::TcpListener;

use crate::repository::newsletter::NewsletterRepository;
use crate::service::newsletter::NewsletterAppService;
use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use routes::api::greet;
use routes::api::healthz;
use routes::api::subscribe;
use std::sync::Arc;

/// Run the server
pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    // create the newsletter repository
    let newsletter_repository = NewsletterRepository::default();
    // create the newsletter app service
    let newsletter_app_service = Arc::new(NewsletterAppService::new(newsletter_repository));

    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::from(newsletter_app_service.clone()))
            .route("/", web::get().to(greet))
            .route("/healthz", web::get().to(healthz))
            .route("/subscriptions", web::post().to(subscribe))
    })
    .listen(listener)?
    .run();
    // return the server
    Ok(server)
}

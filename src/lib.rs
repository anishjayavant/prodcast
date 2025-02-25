//! lib.rs  
//! Declare all of the sub-modules here so Rust can recognize them as part of the crate.
pub mod models;
pub mod routes;
pub mod config;
pub mod app;
use std::net::TcpListener;

use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use routes::api::greet;
use routes::api::healthz;
use routes::api::subscribe;

/// Run the server
pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(greet))
            .route("/healthz", web::get().to(healthz))
            .route("/subscriptions", web::post().to(subscribe))
    })
    .listen(listener)?
    .run();
    // return the server
    Ok(server)
}

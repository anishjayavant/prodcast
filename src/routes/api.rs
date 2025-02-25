use crate::models::newsletter::User;
use crate::app::lang::hello;
use actix_web::{web, HttpRequest, HttpResponse, Responder};

/// Subscribe endpoint
/// This function will subscribe a user to the service
/// It will return a 200 if the subscription is successful
/// It will return a 400 if the subscription is unsuccessful
pub async fn subscribe(_user: web::Form<User>) -> impl Responder {
    // unpack the form data and print
    let str = format!("Hello to Prodcast {}, {}", _user.name(), _user.email());
    HttpResponse::Ok().body(str)
}

/// Health check endpoint
pub async fn healthz() -> impl Responder {
    HttpResponse::Ok().finish()
}

/// Greet the user
pub async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    hello();
    format!("Hello {}!", &name)
}

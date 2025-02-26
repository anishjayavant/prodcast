use crate::models::newsletter::User;
use crate::repository::newsletter::NewsletterPostGresRepository;
use crate::service::lang::hello;
use crate::service::newsletter::{NewsletterAppService, UserService};
use actix_web::{web, HttpRequest, HttpResponse, Responder};

/// Subscribe endpoint
/// This function will subscribe a user to the service
/// It will return a 200 if the subscription is successful
/// It will return a 400 if the subscription is unsuccessful
pub async fn subscribe(
    newsletter_app_service: web::Data<NewsletterAppService<NewsletterPostGresRepository>>,
    user: web::Form<User>,
) -> impl Responder {
    // unpack the form data and print
    let str = format!("Hello to Prodcast {}, {}", user.name(), user.email());
    // save the user to the database
    newsletter_app_service.save_user(user.into_inner()).unwrap();
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

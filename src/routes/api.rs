use crate::models::newsletter::User;
use crate::repository::newsletter::NewsletterPostGresRepository;
use crate::service::lang::hello;
use crate::service::newsletter::NewsletterAppService;
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
    let user_str = format!("User name: {}, email: {}", user.name(), user.email());
    let str = format!("Hello to Prodcast {}, {}", user.name(), user.email());
    // save the user to the database
    log::info!("Saving user: {}", user_str);
    newsletter_app_service
        .save_user(user.into_inner())
        .await
        // return 500 if the user could not be saved or 200 if the user was saved
        .map(|_| {
            log::info!("User saved: {}", user_str);
            HttpResponse::Ok().body(str)
        })
        .unwrap_or_else(|e| {
            log::error!("Failed to save user: {:?}", e);
            HttpResponse::InternalServerError().finish()
        })
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

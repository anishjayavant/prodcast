use crate::models::newsletter::User;
use crate::repository::newsletter::NewsletterPostGresRepository;
use crate::service::lang::hello;
use crate::service::newsletter::NewsletterAppService;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use uuid::Uuid;

/// Subscribe endpoint
/// This function will subscribe a user to the service
/// It will return a 200 if the subscription is successful
/// It will return a 500 if the subscription is unsuccessful
#[tracing::instrument(
    name = "Adding a new subscriber",
    skip(newsletter_app_service, user),
    fields(
        request_id = %Uuid::new_v4(),
        subscriber_email = %user.email(),
        subscriber_name= %user.name()
    )
)]
pub async fn subscribe(
    newsletter_app_service: web::Data<NewsletterAppService<NewsletterPostGresRepository>>,
    user: web::Form<User>,
) -> impl Responder {
    let user_str = format!("User name: {}, email: {}", user.name(), user.email());
    let str = format!("Hello to Prodcast {}, {}", user.name(), user.email());
    // save the user to the database
    tracing::info!("Saving user: {}", user_str);
    newsletter_app_service
        .save_user(user.into_inner())
        .await
        // return 500 if the user could not be saved or 200 if the user was saved
        .map(|_| {
            tracing::info!("User saved: {}", user_str);
            HttpResponse::Ok().body(str)
        })
        .unwrap_or_else(|e| {
            tracing::error!("Failed to save user: {:?}", e);
            HttpResponse::InternalServerError().finish()
        })
}

/// Health check endpoint
#[tracing::instrument(
    name = "Health check",    
    fields(
        request_id = %Uuid::new_v4(),
    )
)]
pub async fn healthz() -> impl Responder {
    tracing::info!("Health check");
    HttpResponse::Ok().finish()
}

/// Greet the user
#[tracing::instrument(
    name = "Greeting user",
    fields(
        request_id = %Uuid::new_v4(),
    )
)]
pub async fn greet(req: HttpRequest) -> impl Responder {
    tracing::info!("Greeting user");
    let name = req.match_info().get("name").unwrap_or("World");
    hello();
    format!("Hello {}!", &name)
}

use crate::models::newsletter::User;
use crate::repository::newsletter::NewsletterPostGresRepository;
use crate::service::lang::hello;
use crate::service::newsletter::NewsletterAppService;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use uuid::Uuid;

/// Subscribe endpoint
/// This function will subscribe a user to the service
/// It will return a 200 if the subscription is successful
/// It will return a 400 if the subscription is unsuccessful
pub async fn subscribe(
    newsletter_app_service: web::Data<NewsletterAppService<NewsletterPostGresRepository>>,
    user: web::Form<User>,
) -> impl Responder {
    // Generate a request ID
    let request_id = Uuid::new_v4();
    // open a new span
    let request_span = tracing::info_span!(
            "Adding a new subscriber",
        %request_id,
        subscriber_name = %user.name(),
        subscriber_email = %user.email()
    );
    // enter the span
    let _request_span_guard = request_span.enter();

    // unpack the form data and print
    let user_str = format!("User name: {}, email: {}", user.name(), user.email());
    let str = format!("Hello to Prodcast {}, {}", user.name(), user.email());
    // save the user to the database
    tracing::info!("request id: {}, saving user: {}", request_id, user_str);
    newsletter_app_service
        .save_user(user.into_inner())
        .await
        // return 500 if the user could not be saved or 200 if the user was saved
        .map(|_| {
            tracing::info!("request id: {}, user saved: {}", request_id, user_str);
            HttpResponse::Ok().body(str)
        })
        .unwrap_or_else(|e| {
            tracing::error!("request id: {}, failed to save user: {:?}", request_id, e);
            HttpResponse::InternalServerError().finish()
        })
}

/// Health check endpoint
pub async fn healthz() -> impl Responder {
    let request_id = Uuid::new_v4();
    tracing::info!("request_id: {}, health check", request_id);
    HttpResponse::Ok().finish()
}

/// Greet the user
pub async fn greet(req: HttpRequest) -> impl Responder {
    let request_id = Uuid::new_v4();
    tracing::info!("request_id: {}, greeting user", request_id);
    let name = req.match_info().get("name").unwrap_or("World");
    hello();
    format!("Hello {}!", &name)
}

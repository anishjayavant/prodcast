use async_trait::async_trait;
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

/// Repository for newsletter to handle all the database operations
/// This module will handle all the database operations for the newsletter
// add a trait to save the user
use crate::models::newsletter::User;

#[async_trait]
pub trait NewsletterRepository {
    // Save the user to the repository
    async fn save_user(&self, user: User) -> Result<(), String>;
    // Get the user from the repository
    async fn get_user(&self, email: String) -> Result<User, String>;
}

pub struct NewsletterPostGresRepository {
    connection_pool: PgPool,
}

impl NewsletterPostGresRepository {
    pub fn new(connection_pool: PgPool) -> Self {
        NewsletterPostGresRepository { connection_pool }
    }
}

/// Implementation of the UserRepository trait using Postgres
#[async_trait]
impl NewsletterRepository for NewsletterPostGresRepository {
    /// Save the user to the repository
    async fn save_user(&self, user: User) -> Result<(), String> {
        // save the user to the database
        tracing::info!("Saving the user {} to the database", user.email());
        sqlx::query!(
            r#"
            INSERT INTO subscriptions (id, email, name, subscribed_at)
            VALUES ($1, $2, $3, $4)
            "#,
            Uuid::new_v4(),
            user.email(),
            user.name(),
            Utc::now()
        )
        .execute(&self.connection_pool)
        .await
        .map(|_| {
            tracing::info!(
                "User with name {} and email {} saved successfully",
                user.name(),
                user.email()
            );
        })
        .map_err(|e| {
            tracing::error!("Failed to execute query: {:?}", e);
            format!(
                "Failed to save user with name {} and email {}",
                user.name(),
                user.email()
            )
        })
    }

    /// Get the user from the repository
    async fn get_user(&self, email: String) -> Result<User, String> {
        // get the user from the database
        tracing::info!("Getting the user {} from the database", email);
        sqlx::query!("SELECT email, name FROM subscriptions",)
            .fetch_one(&self.connection_pool)
            .await
            .map(|row| User::new(row.email, row.name))
            .map_err(|e| {
                tracing::error!("Failed to execute query: {:?}", e);
                "Failed to get user".to_string()
            })
    }
}

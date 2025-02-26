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
        println!("Saving the user {} to the database", user.email());
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
        // We use `get_ref` to get an immutable reference to the `PgConnection`
        // wrapped by `web::Data`.
        .execute(&self.connection_pool)
        .await
        .map_err(|e| {
            eprintln!("Failed to execute query: {:?}", e);
            "Failed to save user".to_string()
        })?;
        Ok(())
    }

    /// Get the user from the repository
    async fn get_user(&self, email: String) -> Result<User, String> {
        // get the user from the database
        println!("Getting the user {} from the database", email);
        let saved = sqlx::query!("SELECT email, name FROM subscriptions",)
            .fetch_one(&self.connection_pool)
            .await
            .map_err(|e| {
                eprintln!("Failed to execute query: {:?}", e);
                "Failed to get user".to_string()
            })?;
        Ok(User::new(saved.email, saved.name))
    }
}

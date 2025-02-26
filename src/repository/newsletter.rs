use sqlx::PgPool;

/// Repository for newsletter to handle all the database operations
/// This module will handle all the database operations for the newsletter
// add a trait to save the user
use crate::models::newsletter::User;

pub trait NewsletterRepository {
    // Save the user to the repository
    fn save_user(&self, user: User) -> Result<(), String>;
    // Get the user from the repository
    fn get_user(&self, email: String) -> Result<User, String>;
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
impl NewsletterRepository for NewsletterPostGresRepository {
    /// Save the user to the repository
    fn save_user(&self, user: User) -> Result<(), String> {
        // save the user to the database
        println!("Saving the user {} to the database", user.email());
        Ok(())
    }

    /// Get the user from the repository
    fn get_user(&self, email: String) -> Result<User, String> {
        // get the user from the database
        println!("Getting the user {} from the database", email);
        Ok(User::new(email, "Test".to_string()))
    }
}

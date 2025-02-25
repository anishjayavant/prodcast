/// Repository for newsletter to handle all the database operations
/// This module will handle all the database operations for the newsletter
// add a trait to save the user
use crate::models::newsletter::User;

pub trait UserRepository {
    // Save the user to the repository
    fn save_user(&self, user: User) -> Result<(), String>;
    // Get the user from the repository
    fn get_user(&self, email: String) -> Result<User, String>;
}

// TODO: This will be created with a PG connection pool eventually
#[derive(Default)]
pub struct NewsletterRepository {}

/// Implementation of the UserRepository trait using Postgres
impl UserRepository for NewsletterRepository {
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

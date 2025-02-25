/// handle all business logic for newsletter
// add a trait to save the user that calls the repository to save the user
use crate::models::newsletter::User;
use crate::repository::newsletter::{NewsletterRepository, UserRepository};

pub trait UserService {
    // Save the user to the repository
    fn save_user(&self, user: User) -> Result<(), String>;
    // Get the user from the repository
    fn get_user(&self, email: String) -> Result<User, String>;
}

pub struct NewsletterAppService {
    repository: NewsletterRepository,
}

impl NewsletterAppService {
    pub fn new(repository: NewsletterRepository) -> Self {
        NewsletterAppService { repository }
    }
}

impl UserService for NewsletterAppService {
    /// Save the user to the repository
    fn save_user(&self, user: User) -> Result<(), String> {
        self.repository.save_user(user)
    }

    /// Get the user from the repository
    fn get_user(&self, email: String) -> Result<User, String> {
        self.repository.get_user(email)
    }
}

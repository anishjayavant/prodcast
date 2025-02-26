/// handle all business logic for newsletter
// add a trait to save the user that calls the repository to save the user
use crate::models::newsletter::User;
use crate::repository::newsletter::NewsletterRepository;

pub trait UserService {
    // Save the user to the repository
    fn save_user(&self, user: User) -> Result<(), String>;
    // Get the user from the repository
    fn get_user(&self, email: String) -> Result<User, String>;
}

// struct to handle all the business logic for the newsletter
pub struct NewsletterAppService<T: NewsletterRepository> {
    repository: T,
}

impl<T: NewsletterRepository> NewsletterAppService<T> {
    pub fn new(repository: T) -> Self {
        NewsletterAppService { repository }
    }
}

impl<T: NewsletterRepository> UserService for NewsletterAppService<T> {
    /// Save the user to the repository
    fn save_user(&self, user: User) -> Result<(), String> {
        self.repository.save_user(user)
    }

    /// Get the user from the repository
    fn get_user(&self, email: String) -> Result<User, String> {
        self.repository.get_user(email)
    }
}

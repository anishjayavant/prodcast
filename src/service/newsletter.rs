/// handle all business logic for newsletter
// add a trait to save the user that calls the repository to save the user
use crate::models::newsletter::User;
use crate::repository::newsletter::NewsletterRepository;

// struct to handle all the business logic for the newsletter
pub struct NewsletterAppService<T: NewsletterRepository> {
    repository: T,
}

impl<T: NewsletterRepository> NewsletterAppService<T> {
    pub fn new(repository: T) -> Self {
        NewsletterAppService { repository }
    }
}

impl<T: NewsletterRepository> NewsletterAppService<T> {
    /// Save the user to the repository
    pub async fn save_user(&self, user: User) -> Result<(), String> {
        self.repository.save_user(user).await.map_err(|e| {
            log::error!("Failed to save user: {:?}", e);
            "Failed to save user".to_string()
        })
    }

    /// Get the user from the repository
    pub async fn get_user(&self, email: String) -> Result<User, String> {
        self.repository.get_user(email).await.map_err(|e| {
            log::error!("Failed to get user: {:?}", e);
            "Failed to get user".to_string()
        })
    }
}

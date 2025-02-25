#[derive(serde::Deserialize)]
pub struct User {
    email: String,
    name: String,
}

impl User {
    pub fn new(email: String, name: String) -> Self {
        User { email, name }
    }

    pub fn email(&self) -> &str {
        &self.email
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

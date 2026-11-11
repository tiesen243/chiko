use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct User {
    id: u32,
    username: String,
    email: String,
}

impl User {
    pub fn new(id: u32, username: String, email: String) -> Self {
        User {
            id,
            username,
            email,
        }
    }

    // Getters
    pub fn get_id(&self) -> u32 {
        self.id
    }
    pub fn get_username(&self) -> &str {
        &self.username
    }
    pub fn get_email(&self) -> &str {
        &self.email
    }

    // Setters
    pub fn set_username(&mut self, username: String) {
        self.username = username;
    }
    pub fn set_email(&mut self, email: String) {
        self.email = email;
    }
}

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Post {
    id: u32,
    title: String,
    content: String,
}

impl Post {
    pub fn new(id: u32, title: String, content: String) -> Self {
        Post { id, title, content }
    }

    // Getters
    pub fn get_id(&self) -> u32 {
        self.id
    }
    pub fn get_title(&self) -> &str {
        &self.title
    }
    pub fn get_content(&self) -> &str {
        &self.content
    }

    // Setters
    pub fn set_title(&mut self, title: String) {
        self.title = title;
    }
    pub fn set_content(&mut self, content: String) {
        self.content = content;
    }
}

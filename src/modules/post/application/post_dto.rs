use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreatePostDto {
    pub id: u32,
    pub title: String,
    pub content: String,
}

#[derive(Deserialize)]
pub struct UpdatePostDto {
    pub title: Option<String>,
    pub content: Option<String>,
}

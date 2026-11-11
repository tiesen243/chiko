use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct CreatePostDto {
    pub title: String,
    pub content: String,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct UpdatePostDto {
    pub title: Option<String>,
    pub content: Option<String>,
}

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct CreateUserDto {
    pub username: String,
    pub email: String,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct UpdateUserDto {
    pub username: Option<String>,
    pub email: Option<String>,
}

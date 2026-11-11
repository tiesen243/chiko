use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateUserDto {
    pub id: u32,
    pub username: String,
    pub email: String,
}

#[derive(Deserialize)]
pub struct UpdateUserDto {
    pub username: Option<String>,
    pub email: Option<String>,
}

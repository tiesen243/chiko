use worker::{D1Database, D1Type, Result};

use crate::modules::user::{
    application::{CreateUserDto, UpdateUserDto},
    domain::User,
};

pub struct UserService {
    db: D1Database,
}

impl UserService {
    pub fn new(db: D1Database) -> Self {
        Self { db }
    }

    pub async fn all(&self) -> Result<Vec<User>> {
        let result = self
            .db
            .prepare("SELECT id, username, email FROM users ORDER BY id")
            .all()
            .await?;

        result.results()
    }

    pub async fn one(&self, id: u32) -> Result<Option<User>> {
        let id = D1Type::Integer(id as i32);

        self.db
            .prepare("SELECT id, username, email FROM users WHERE id = ?")
            .bind_refs([&id])?
            .first(None)
            .await
    }

    pub async fn create(&self, data: CreateUserDto) -> Result<()> {
        let username = D1Type::Text(&data.username);
        let email = D1Type::Text(&data.email);

        self.db
            .prepare("INSERT INTO users (username, email) VALUES (?, ?)")
            .bind_refs([&username, &email])?
            .run()
            .await?;

        Ok(())
    }

    pub async fn update(&self, id: u32, data: UpdateUserDto) -> Result<()> {
        let id = D1Type::Integer(id as i32);

        let username = data
            .username
            .as_deref()
            .map(D1Type::Text)
            .unwrap_or(D1Type::Null);

        let email = data
            .email
            .as_deref()
            .map(D1Type::Text)
            .unwrap_or(D1Type::Null);

        self.db
            .prepare(
                "UPDATE users
                 SET username = COALESCE(?, username),
                     email = COALESCE(?, email)
                 WHERE id = ?",
            )
            .bind_refs([&username, &email, &id])?
            .run()
            .await?;

        Ok(())
    }

    pub async fn delete(&self, id: u32) -> Result<()> {
        let id = D1Type::Integer(id as i32);

        self.db
            .prepare("DELETE FROM users WHERE id = ?")
            .bind_refs([&id])?
            .run()
            .await?;

        Ok(())
    }
}

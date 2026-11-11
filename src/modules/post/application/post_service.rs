use worker::{D1Database, D1Type, Result};

use crate::modules::post::{
    application::{CreatePostDto, UpdatePostDto},
    domain::Post,
};

pub struct PostService {
    db: D1Database,
}

impl PostService {
    pub fn new(db: D1Database) -> Self {
        Self { db }
    }

    pub async fn all(&self) -> Result<Vec<Post>> {
        let result = self
            .db
            .prepare("SELECT id, title, content FROM posts ORDER BY id")
            .all()
            .await?;

        result.results()
    }

    pub async fn one(&self, id: u32) -> Result<Option<Post>> {
        let id = D1Type::Integer(id as i32);

        self.db
            .prepare("SELECT id, title, content FROM posts WHERE id = ?")
            .bind_refs([&id])?
            .first(None)
            .await
    }

    pub async fn create(&self, data: CreatePostDto) -> Result<()> {
        let title = D1Type::Text(&data.title);
        let content = D1Type::Text(&data.content);

        self.db
            .prepare("INSERT INTO posts (title, content) VALUES (?, ?)")
            .bind_refs([&title, &content])?
            .run()
            .await?;

        Ok(())
    }

    pub async fn update(&self, id: u32, data: UpdatePostDto) -> Result<()> {
        let id = D1Type::Integer(id as i32);

        let title = data
            .title
            .as_deref()
            .map(D1Type::Text)
            .unwrap_or(D1Type::Null);

        let content = data
            .content
            .as_deref()
            .map(D1Type::Text)
            .unwrap_or(D1Type::Null);

        self.db
            .prepare(
                "UPDATE posts
                 SET title = COALESCE(?, title),
                     content = COALESCE(?, content)
                 WHERE id = ?",
            )
            .bind_refs([&title, &content, &id])?
            .run()
            .await?;

        Ok(())
    }

    pub async fn delete(&self, id: u32) -> Result<()> {
        let id = D1Type::Integer(id as i32);

        self.db
            .prepare("DELETE FROM posts WHERE id = ?")
            .bind_refs([&id])?
            .run()
            .await?;

        Ok(())
    }
}

use crate::model::{NewPost, Post};
use sqlx::SqlitePool;
use std::future::Future;

pub trait PostRepo: Clone + Send + Sync + 'static {
    fn list(&self) -> impl Future<Output = sqlx::Result<Vec<Post>>> + Send;

    fn get(&self, id: i64) -> impl Future<Output = sqlx::Result<Option<Post>>> + Send;

    fn create(&self, post: NewPost) -> impl Future<Output = sqlx::Result<Post>> + Send;

    fn update(
        &self,
        id: i64,
        new_post: NewPost,
    ) -> impl Future<Output = sqlx::Result<Option<Post>>> + Send;

    fn delete(&self, id: i64) -> impl Future<Output = sqlx::Result<bool>> + Send;
}

#[derive(Clone)]
pub struct SqlitePostRepo {
    pub pool: SqlitePool,
}

impl PostRepo for SqlitePostRepo {
    async fn list(&self) -> sqlx::Result<Vec<Post>> {
        sqlx::query_as!(
            Post,
            "SELECT id, title, body, created_at FROM posts ORDER BY id"
        )
        .fetch_all(&self.pool)
        .await
    }

    async fn get(&self, id: i64) -> sqlx::Result<Option<Post>> {
        sqlx::query_as!(
            Post,
            "SELECT id, title, body, created_at FROM posts WHERE id = ?",
            id
        )
        .fetch_optional(&self.pool)
        .await
    }

    async fn create(&self, post: NewPost) -> sqlx::Result<Post> {
        sqlx::query_as!(
            Post,
            r#"
            INSERT INTO posts (title, body)
            VALUES (?, ?)
            RETURNING id, title, body, created_at
            "#,
            post.title.clone(),
            post.body.clone()
        )
        .fetch_one(&self.pool)
        .await
    }

    async fn update(&self, id: i64, new_post: NewPost) -> sqlx::Result<Option<Post>> {
        sqlx::query_as!(
            Post,
            r#"
            UPDATE posts
            SET title = ?, body = ?
            WHERE id = ?
            RETURNING id, title, body, created_at
            "#,
            new_post.title,
            new_post.body,
            id
        )
        .fetch_optional(&self.pool)
        .await
    }

    async fn delete(&self, id: i64) -> sqlx::Result<bool> {
        let result = sqlx::query!("DELETE FROM posts WHERE id = ?", id)
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }
}

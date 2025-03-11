use crate::domain::interfaces::content_interface::ContentInterface;
use crate::domain::models::content::Content;
use crate::types::{BoxError, DbPool};
use async_trait::async_trait;

#[derive(Clone)]
pub struct ContentRepository {
    pub pool: DbPool,
}

impl ContentRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ContentInterface for ContentRepository {
    async fn create(&self, content: Content) -> Result<Content, BoxError> {
        Ok(sqlx::query_as::<_, Content>(
            "INSERT INTO contents (title, body) VALUES ($1, $2) RETURNING *",
        )
        .bind(&content.title)
        .bind(&content.body)
        .fetch_one(&self.pool)
        .await?)
    }
    async fn find(&self, id: i64) -> Result<Option<Content>, BoxError> {
        Ok(
            sqlx::query_as::<_, Content>("SELECT * FROM contents WHERE id = $1")
                .bind(id)
                .fetch_optional(&self.pool)
                .await?,
        )
    }
}

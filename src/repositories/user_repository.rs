use crate::domain::interfaces::user_interface::UserInterface;
use crate::domain::models::user::User;
use crate::types::{BoxError, DbPool};
use async_trait::async_trait;

#[derive(Clone)]
pub struct UserRepository {
    pub pool: DbPool,
}

impl UserRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserInterface for UserRepository {
    async fn find_by_username(&self, username: String) -> Result<Option<User>, BoxError> {
        Ok(
            sqlx::query_as::<_, User>("SELECT * FROM users WHERE username = $1")
                .bind(username)
                .fetch_optional(&self.pool)
                .await?,
        )
    }
}

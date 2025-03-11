use crate::domain::models::user::User;
use crate::types::BoxError;
use async_trait::async_trait;

#[async_trait]
pub trait UserInterface {
    async fn find_by_username(&self, username: String) -> Result<Option<User>, BoxError>;
}

use crate::domain::models::content::Content;
use crate::types::BoxError;
use async_trait::async_trait;

#[async_trait]
pub trait ContentInterface {
    async fn create(&self, content: Content) -> Result<Content, BoxError>;
    async fn find(&self, id: i64) -> Result<Option<Content>, BoxError>;
}

use crate::domain::interfaces::content_interface::ContentInterface;
use crate::domain::models::content::Content;
use crate::types::BoxError;
use async_trait::async_trait;

#[derive(Clone)]
pub struct ContentUseCase<T: ContentInterface + Clone> {
    repository: T,
}

impl<T: ContentInterface + Clone> ContentUseCase<T> {
    pub fn new(repository: T) -> Self {
        Self { repository }
    }
}

#[async_trait]
pub trait ContentService {
    async fn create(&self, content: Content) -> Result<Content, BoxError>;
    async fn find(&self, id: i64) -> Result<Option<Content>, BoxError>;
}

#[async_trait]
impl<T: ContentInterface + Send + Sync + Clone> ContentService for ContentUseCase<T> {
    async fn create(&self, content: Content) -> Result<Content, BoxError> {
        self.repository.create(content).await
    }
    async fn find(&self, id: i64) -> Result<Option<Content>, BoxError> {
        self.repository.find(id).await
    }
}

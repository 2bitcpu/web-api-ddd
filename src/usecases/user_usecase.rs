use crate::domain::interfaces::user_interface::UserInterface;
use crate::domain::models::user::LoginUserRequest;
use crate::types::BoxError;
use async_trait::async_trait;

#[derive(Clone)]
pub struct UserUseCase<T: UserInterface + Clone> {
    repository: T,
}

impl<T: UserInterface + Clone> UserUseCase<T> {
    pub fn new(repository: T) -> Self {
        Self { repository }
    }
}

#[async_trait]
pub trait UserService {
    async fn signin(&self, payload: LoginUserRequest) -> Result<Option<String>, BoxError>;
}

#[async_trait]
impl<T: UserInterface + Send + Sync + Clone> UserService for UserUseCase<T> {
    async fn signin(&self, payload: LoginUserRequest) -> Result<Option<String>, BoxError> {
        let user = self.repository.find_by_username(payload.username).await?;
        match user {
            Some(user) => {
                if user.password == payload.password {
                    Ok(Some(simple_jwt::encode(&simple_jwt::Claims::new(
                        &user.username,
                        3600,
                    ))?))
                } else {
                    Ok(None)
                }
            }
            None => Ok(None),
        }
    }
}

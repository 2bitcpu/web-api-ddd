use crate::handlers::content_handler::create_content_router;
use crate::handlers::user_handler::create_user_router;
use crate::repositories::content_repository::ContentRepository;
use crate::types::DbPool;
use crate::usecases::content_usecase::ContentUseCase;
use axum::Router;

pub fn routes(pool: DbPool) -> Router {
    let content_repository = ContentRepository::new(pool.clone());
    let content_service = ContentUseCase::new(content_repository);

    let user_repository = crate::repositories::user_repository::UserRepository::new(pool.clone());
    let user_service = crate::usecases::user_usecase::UserUseCase::new(user_repository);

    Router::new()
        .nest("/auth", create_user_router(user_service))
        .nest("/contents", create_content_router(content_service))
}

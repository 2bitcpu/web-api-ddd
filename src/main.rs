use crate::handlers::create_handler;
use crate::typs::BoxError;
use sqlx::SqlitePool;
use tokio::net::TcpListener;

use axum::Router;

#[tokio::main]
async fn main() -> Result<(), BoxError> {
    let pool = SqlitePool::connect("sqlite:./data.db").await?;

    let app = Router::new().nest("/service", create_handler::routes(pool.clone()));

    let listener = TcpListener::bind("0.0.0.0:3000").await?;

    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await?;

    Ok(())
}

pub mod typs {
    use sqlx::SqlitePool;

    pub type BoxError = Box<dyn std::error::Error + Send + Sync>;

    pub type DbPool = SqlitePool;
}

pub mod domain {
    pub mod models {
        pub mod content {
            use chrono::{DateTime, Utc};
            use serde::{Deserialize, Serialize};
            use sqlx::FromRow;

            #[derive(Debug, Serialize, Deserialize, FromRow)]
            pub struct Content {
                pub id: i64,
                pub title: String,
                pub body: String,
                pub created_at: Option<DateTime<Utc>>,
                pub updated_at: Option<DateTime<Utc>>,
            }

            impl Content {
                pub fn new(title: String, body: String) -> Self {
                    Self {
                        id: 0,
                        title,
                        body,
                        created_at: None,
                        updated_at: None,
                    }
                }
            }

            #[derive(Deserialize)]
            pub struct CreateContentRequest {
                pub title: String,
                pub body: String,
            }
        }

        pub mod user {
            use serde::{Deserialize, Serialize};
            use sqlx::FromRow;

            #[derive(Debug, Serialize, Deserialize, FromRow)]
            pub struct User {
                pub id: i64,
                pub username: String,
                pub password: String,
            }

            #[derive(Deserialize)]
            pub struct LoginUserRequest {
                pub username: String,
                pub password: String,
            }
        }
    }

    pub mod interfaces {
        pub mod content_interface {
            use crate::domain::models::content::Content;
            use crate::typs::BoxError;
            use async_trait::async_trait;

            #[async_trait]
            pub trait ContentInterface {
                async fn create(&self, content: Content) -> Result<Content, BoxError>;
                async fn find(&self, id: i64) -> Result<Option<Content>, BoxError>;
            }
        }

        pub mod user_interface {
            use crate::domain::models::user::User;
            use crate::typs::BoxError;
            use async_trait::async_trait;

            #[async_trait]
            pub trait UserInterface {
                async fn find_by_username(
                    &self,
                    username: String,
                ) -> Result<Option<User>, BoxError>;
            }
        }
    }
}

pub mod repositories {
    pub mod content_repository {
        use crate::domain::interfaces::content_interface::ContentInterface;
        use crate::domain::models::content::Content;
        use crate::typs::{BoxError, DbPool};
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
    }

    pub mod user_repository {
        use crate::domain::interfaces::user_interface::UserInterface;
        use crate::domain::models::user::User;
        use crate::typs::{BoxError, DbPool};
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
    }
}

pub mod usecases {
    pub mod content_usecase {

        use crate::domain::interfaces::content_interface::ContentInterface;
        use crate::domain::models::content::Content;
        use crate::typs::BoxError;
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
    }

    pub mod user_usecase {
        use crate::domain::interfaces::user_interface::UserInterface;
        use crate::domain::models::user::LoginUserRequest;
        use crate::typs::BoxError;
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
                            Ok(Some(user.username))
                        } else {
                            Ok(None)
                        }
                    }
                    None => Ok(None),
                }
            }
        }
    }
}

pub mod handlers {
    pub mod create_handler {
        use crate::handlers::content_handler::create_content_router;
        use crate::handlers::user_handler::create_user_router;
        use crate::repositories::content_repository::ContentRepository;
        use crate::typs::DbPool;
        use crate::usecases::content_usecase::ContentUseCase;
        use axum::Router;

        pub fn routes(pool: DbPool) -> Router {
            let content_repository = ContentRepository::new(pool.clone());
            let content_service = ContentUseCase::new(content_repository);

            let user_repository =
                crate::repositories::user_repository::UserRepository::new(pool.clone());
            let user_service = crate::usecases::user_usecase::UserUseCase::new(user_repository);

            Router::new()
                .nest("/auth", create_user_router(user_service))
                .nest("/contents", create_content_router(content_service))
        }
    }
    pub mod content_handler {

        use crate::domain::models::content::{Content, CreateContentRequest};
        use crate::usecases::content_usecase::ContentService;
        use axum::extract::{Path, State};
        use axum::response::IntoResponse;
        use axum::{
            Router,
            extract::Json,
            http::StatusCode,
            routing::{get, post},
        };
        use serde_json::json;
        use std::sync::Arc;

        #[derive(Clone)]
        pub struct AppState<T: ContentService> {
            pub content_service: Arc<T>,
        }

        async fn create_content<T: ContentService>(
            State(state): State<AppState<T>>,
            Json(payload): Json<CreateContentRequest>,
        ) -> impl IntoResponse {
            match state
                .content_service
                .create(Content::new(payload.title, payload.body))
                .await
            {
                Ok(content) => (StatusCode::OK, Json(content)).into_response(),
                Err(err) => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"error": err.to_string()})),
                )
                    .into_response(),
            }
        }

        async fn find_content<T: ContentService>(
            State(state): State<AppState<T>>,
            Path(id): Path<i64>,
        ) -> impl IntoResponse {
            match state.content_service.find(id).await {
                Ok(content) => (StatusCode::OK, Json(content)).into_response(),
                Err(err) => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"error": err.to_string()})),
                )
                    .into_response(),
            }
        }

        pub fn create_content_router<T: ContentService + Send + Sync + 'static + Clone>(
            content_service: T,
        ) -> Router {
            let state = AppState {
                content_service: Arc::new(content_service),
            };

            Router::new()
                .route("/post", post(create_content::<T>))
                .route("/find/{id}", get(find_content::<T>))
                .with_state(state)
        }
    }

    pub mod user_handler {
        use crate::usecases::user_usecase::UserService;
        use axum::extract::State;
        use axum::response::IntoResponse;
        use axum::{Router, extract::Json, http::StatusCode, routing::post};
        use serde_json::json;
        use std::sync::Arc;

        #[derive(Clone)]
        pub struct AppState<T: UserService> {
            pub user_service: Arc<T>,
        }

        async fn signin<T: UserService>(
            State(state): State<AppState<T>>,
            Json(payload): Json<crate::domain::models::user::LoginUserRequest>,
        ) -> impl IntoResponse {
            match state.user_service.signin(payload).await {
                Ok(user) => (StatusCode::OK, Json(user)).into_response(),
                Err(err) => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({"error": err.to_string()})),
                )
                    .into_response(),
            }
        }

        pub fn create_user_router<T: UserService + Send + Sync + 'static + Clone>(
            user_service: T,
        ) -> Router {
            let state = AppState {
                user_service: Arc::new(user_service),
            };

            Router::new()
                .route("/signin", post(signin::<T>))
                .with_state(state)
        }
    }
}

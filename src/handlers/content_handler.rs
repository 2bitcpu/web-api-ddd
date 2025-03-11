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

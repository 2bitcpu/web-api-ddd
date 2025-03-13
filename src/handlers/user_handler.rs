use crate::domain::models::user::LoginUserRequest;
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
    Json(payload): Json<LoginUserRequest>,
) -> impl IntoResponse {
    match state.user_service.signin(payload).await {
        Ok(token) => (StatusCode::OK, Json(serde_json::json!({"token": token}))).into_response(),
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

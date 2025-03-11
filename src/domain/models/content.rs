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

use sqlx::SqlitePool;

pub type BoxError = Box<dyn std::error::Error + Send + Sync>;

pub type DbPool = SqlitePool;

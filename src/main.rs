use sqlx::SqlitePool;
use tokio::net::TcpListener;
use web_api::handlers::create_handler;
use web_api::types::BoxError;

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

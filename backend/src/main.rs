pub mod api;
pub mod db;
pub mod models;
pub mod state;

use axum::{Router, routing::get};
use dotenvy::dotenv;

use crate::{api::item, state::AppState};

#[tokio::main]
async fn main() {
    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());

    let pool = db::connect(&database_url).await;
    let state = AppState { db: pool };

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/items", get(item::get_items).post(item::create_item))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{port}"))
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

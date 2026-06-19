use dotenvy::dotenv;
use std::env;

use backend::{create_router, db, external::client::ApiClient, state::AppState};

#[tokio::main]
async fn main() {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let tmdb_access_token = env::var("TMDB_ACCESS_TOKEN").expect("TMDB_ACCESS_TOKEN must be set");

    let pool = db::connect(&database_url)
        .await
        .expect("failed to connect to database");
    let client = ApiClient::new(tmdb_access_token);
    let state = AppState { db: pool, client };

    let app = create_router(state);

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{port}"))
        .await
        .expect("failed to bind to port");

    axum::serve(listener, app).await.expect("server error");
}

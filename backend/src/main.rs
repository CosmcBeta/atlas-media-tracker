use dotenvy::dotenv;

use backend::{create_router, db, state::AppState};

#[tokio::main]
async fn main() {
    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());

    let pool = db::connect(&database_url)
        .await
        .expect("failed to connect to database");
    let state = AppState { db: pool };

    let app = create_router(state);

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{port}"))
        .await
        .expect("failed to bind to port");

    axum::serve(listener, app).await.expect("server error");
}

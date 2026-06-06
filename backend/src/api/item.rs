use axum::Json;
use axum::http::StatusCode;
use axum::{extract::State, response::IntoResponse};
use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::{
    models::item::{Item, MediaType},
    state::AppState,
};

pub async fn get_items(State(state): State<AppState>) -> impl IntoResponse {
    // use the non macro version if we don't have a guarenteed string like the one below, basically if we have filters
    let items = sqlx::query_as!(
        Item,
        r#"SELECT
        id AS "id!: Uuid",
        media_type AS "media_type: MediaType",
        title,
        external_id,
        metadata,
        created_at as "created_at: DateTime<Utc>",
        updated_at as "updated_at: DateTime<Utc>"
        FROM items"#
    )
    .fetch_all(&state.db)
    .await
    .unwrap();

    Json(items)
}
pub async fn create_item(State(state): State<AppState>) -> impl IntoResponse {
    sqlx::query!(
        "INSERT INTO items (id, media_type, title, external_id, metadata, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?)",
        Uuid::new_v4(), MediaType::Show, "One Piece".to_string(), None::<String>, None::<String>, Utc::now().to_rfc3339(), Utc::now().to_rfc3339()
    ).execute(&state.db).await.unwrap();

    StatusCode::CREATED // can also return the created item, but nothing rn
}
pub async fn get_item(State(state): State<AppState>) -> impl IntoResponse {}
pub async fn update_item(State(state): State<AppState>) -> impl IntoResponse {}
pub async fn delete_item(State(state): State<AppState>) -> impl IntoResponse {}

// async fn items_index(State(db): State<Db>) -> impl IntoResponse {
//     let items = db.read().unwrap();

//     let items = items.values().cloned().collect::<Vec<_>>();

//     Json(items)
// }

// async fn items_create(State(db): State<Db>) -> impl IntoResponse {
//     let item = Item {
//         id: Uuid::new_v4(),
//         text: "Hey man".to_string(),
//     };

//     db.write().unwrap().insert(item.id, item.clone());

//     (StatusCode::CREATED, Json(item))
// }

use axum::{extract::State, response::IntoResponse};

use crate::state::AppState;

pub async fn get_item_progress(State(state): State<AppState>) -> impl IntoResponse {}
pub async fn update_item_progress(State(state): State<AppState>) -> impl IntoResponse {}
pub async fn delete_item_progress(State(state): State<AppState>) -> impl IntoResponse {}

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

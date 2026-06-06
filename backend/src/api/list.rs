use axum::{extract::State, response::IntoResponse};

use crate::state::AppState;

pub async fn get_lists(State(state): State<AppState>) -> impl IntoResponse {}
pub async fn create_list(State(state): State<AppState>) -> impl IntoResponse {}
pub async fn update_list(State(state): State<AppState>) -> impl IntoResponse {}
pub async fn delete_list(State(state): State<AppState>) -> impl IntoResponse {}
pub async fn get_list_items(State(state): State<AppState>) -> impl IntoResponse {}
pub async fn add_item_to_list(State(state): State<AppState>) -> impl IntoResponse {}
pub async fn delete_item_from_list(State(state): State<AppState>) -> impl IntoResponse {}

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

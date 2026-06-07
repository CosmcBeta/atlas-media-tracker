use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use chrono::{DateTime, Utc};
use sqlx::{Arguments, AssertSqlSafe, query, sqlite::SqliteArguments};
use uuid::Uuid;

use crate::{
    models::item::{CreateItem, Item, MediaType, UpdateItem},
    state::AppState,
};

pub async fn get_items(State(state): State<AppState>) -> impl IntoResponse {
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

pub async fn create_item(
    State(state): State<AppState>,
    Json(input): Json<CreateItem>,
) -> impl IntoResponse {
    let item = Item {
        id: Uuid::new_v4(),
        media_type: input.media_type,
        title: input.title,
        external_id: None, // none for now until we get external api
        metadata: None,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    sqlx::query!(
        r#"INSERT INTO items (id, media_type, title, external_id, metadata, created_at, updated_at)
        VALUES (?, ?, ?, ?, ?, ?, ?)"#,
        item.id,
        item.media_type,
        item.title,
        item.external_id,
        item.metadata,
        item.created_at.to_rfc3339(),
        item.updated_at.to_rfc3339()
    )
    .execute(&state.db)
    .await
    .unwrap();

    (StatusCode::CREATED, Json(item))
}

pub async fn get_item(Path(id): Path<Uuid>, State(state): State<AppState>) -> impl IntoResponse {
    let item = sqlx::query_as!(
        Item,
        r#"SELECT
        id AS "id!: Uuid",
        media_type AS "media_type: MediaType",
        title,
        external_id,
        metadata,
        created_at as "created_at: DateTime<Utc>",
        updated_at as "updated_at: DateTime<Utc>"
        FROM items WHERE id = ?"#,
        id
    )
    .fetch_optional(&state.db)
    .await
    .unwrap();

    match item {
        Some(item) => (StatusCode::OK, Json(item)).into_response(),
        None => StatusCode::NOT_FOUND.into_response(),
    }
}

pub async fn update_item(
    Path(id): Path<Uuid>,
    State(state): State<AppState>,
    Json(input): Json<UpdateItem>,
) -> impl IntoResponse {
    let mut sets = Vec::new();
    let mut args = SqliteArguments::default();

    if let Some(title) = input.title {
        sets.push("title = ?");
        let _ = args.add(title);
    }

    if let Some(media_type) = input.media_type {
        sets.push("media_type = ?");
        let _ = args.add(media_type);
    }

    if let Some(external_id) = input.external_id {
        sets.push("external_id = ?");
        let _ = args.add(external_id);
    }

    if let Some(metadata) = input.metadata {
        sets.push("metadata = ?");
        let _ = args.add(metadata);
    }

    if sets.is_empty() {
        return StatusCode::BAD_REQUEST;
    }

    sets.push("updated_at = ?");
    let _ = args.add(Utc::now().to_rfc3339());

    let query = format!("UPDATE items SET {} WHERE id = ?", sets.join(", "));
    let _ = args.add(&id);

    sqlx::query_with(AssertSqlSafe(query), args)
        .execute(&state.db)
        .await
        .unwrap();

    StatusCode::OK
}

pub async fn delete_item(Path(id): Path<Uuid>, State(state): State<AppState>) -> impl IntoResponse {
    query!("DELETE FROM items WHERE id = ?", id)
        .execute(&state.db)
        .await
        .unwrap();

    StatusCode::NO_CONTENT
}

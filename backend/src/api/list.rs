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
    models::{
        item::{Item, MediaType},
        list::{AddItemToList, CreateList, List, UpdateList},
    },
    state::AppState,
};

pub async fn get_lists(State(state): State<AppState>) -> impl IntoResponse {
    let lists = sqlx::query_as!(
        List,
        r#"SELECT
        id AS "id!: Uuid",
        name,
        icon,
        created_at as "created_at: DateTime<Utc>",
        updated_at as "updated_at: DateTime<Utc>"
        FROM lists"#
    )
    .fetch_all(&state.db)
    .await
    .unwrap();

    Json(lists)
}

pub async fn create_list(
    State(state): State<AppState>,
    Json(input): Json<CreateList>,
) -> impl IntoResponse {
    let list = List {
        id: Uuid::new_v4(),
        name: input.name,
        icon: input.icon,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    sqlx::query!(
        r#"INSERT INTO lists (id, name, icon, created_at, updated_at)
        VALUES (?, ?, ?, ?, ?)"#,
        list.id,
        list.name,
        list.icon,
        list.created_at.to_rfc3339(),
        list.updated_at.to_rfc3339()
    )
    .execute(&state.db)
    .await
    .unwrap();

    (StatusCode::CREATED, Json(list))
}

pub async fn update_list(
    Path(id): Path<Uuid>,
    State(state): State<AppState>,
    Json(input): Json<UpdateList>,
) -> impl IntoResponse {
    let mut sets = Vec::new();
    let mut args = SqliteArguments::default();

    if let Some(name) = input.name {
        sets.push("name = ?");
        let _ = args.add(name);
    }

    if let Some(icon) = input.icon {
        sets.push("icon = ?");
        let _ = args.add(icon);
    }

    if sets.is_empty() {
        return StatusCode::BAD_REQUEST;
    }

    sets.push("updated_at = ?");
    let _ = args.add(Utc::now().to_rfc3339());

    let query = format!("UPDATE lists SET {} WHERE id = ?", sets.join(", "));
    let _ = args.add(&id);

    sqlx::query_with(AssertSqlSafe(query), args)
        .execute(&state.db)
        .await
        .unwrap();

    StatusCode::OK
}

pub async fn delete_list(Path(id): Path<Uuid>, State(state): State<AppState>) -> impl IntoResponse {
    let _ = query!("DELETE FROM lists WHERE id = ?", id)
        .execute(&state.db)
        .await
        .unwrap();

    StatusCode::NO_CONTENT
}

pub async fn get_list_items(
    Path(id): Path<Uuid>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let items = sqlx::query_as!(
        Item,
        r#"SELECT
        i.id AS "id!: Uuid",
        i.media_type AS "media_type: MediaType",
        i.title,
        i.external_id,
        i.metadata,
        i.created_at as "created_at: DateTime<Utc>",
        i.updated_at as "updated_at: DateTime<Utc>"
        FROM items i
        INNER JOIN list_items li ON i.id = li.item_id
        WHERE li.list_id = ?"#,
        id
    )
    .fetch_all(&state.db)
    .await
    .unwrap();

    Json(items)
}

pub async fn add_item_to_list(
    Path(id): Path<Uuid>,
    State(state): State<AppState>,
    Json(input): Json<AddItemToList>,
) -> impl IntoResponse {
    sqlx::query!(
        r#"INSERT INTO list_items (list_id, item_id, added_at, sort_order)
        VALUES (?, ?, ?, ?)"#,
        id,
        input.item_id,
        Utc::now().to_rfc3339(),
        0
    )
    .execute(&state.db)
    .await
    .unwrap();

    StatusCode::CREATED
}

pub async fn delete_item_from_list(
    Path((id, item_id)): Path<(Uuid, Uuid)>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    query!(
        "DELETE FROM list_items WHERE list_id = ? AND item_id = ?",
        id,
        item_id
    )
    .execute(&state.db)
    .await
    .unwrap();

    StatusCode::NO_CONTENT
}

use sqlx::SqlitePool;

use crate::external::client::ApiClient;

#[derive(Debug, Clone)]
pub struct AppState {
    pub db: SqlitePool,
    pub client: ApiClient,
}

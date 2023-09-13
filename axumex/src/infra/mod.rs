pub mod db;
pub mod orm;
pub mod signal;

#[derive(Clone)]
pub struct AppState {
    pub db: sqlx::SqlitePool,
    pub conn: sea_orm::DatabaseConnection,
}

#[derive(Clone)]
pub struct ServerState {
    pub event_stream: tokio::sync::broadcast::Sender<String>,
}

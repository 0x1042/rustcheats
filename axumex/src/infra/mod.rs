pub mod db;
pub mod orm;
pub mod signal;

#[derive(Clone)]
pub struct AppState {
    pub db: sqlx::SqlitePool,
    pub conn: sea_orm::DatabaseConnection,
}

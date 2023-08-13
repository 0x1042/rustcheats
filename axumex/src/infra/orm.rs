use sea_orm::DatabaseConnection;
use tokio::sync::OnceCell;

use crate::infra::db::init_pool;

static POOL: OnceCell<DatabaseConnection> = OnceCell::const_new();

pub async fn init_conn() -> DatabaseConnection {
    sea_orm::SqlxSqliteConnector::from_sqlx_sqlite_pool(init_pool().await)
}

pub async fn get_conn() -> &'static DatabaseConnection {
    POOL.get_or_init(init_conn).await
}

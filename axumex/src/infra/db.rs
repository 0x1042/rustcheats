use sqlx::{
    sqlite::{SqliteConnectOptions, SqlitePoolOptions},
    SqlitePool,
};
use tokio::sync::OnceCell;
use tracing::info;

static POOL: OnceCell<SqlitePool> = OnceCell::const_new();

pub async fn init_pool() -> SqlitePool {
    let opt = SqliteConnectOptions::new()
        .filename("all.db")
        .create_if_missing(true);

    let db = SqlitePoolOptions::new()
        .max_connections(1024)
        .connect_with(opt)
        .await
        .expect("create sqlite connect fail");

    info!("init db pool success");
    db
}

pub async fn get_pool() -> &'static SqlitePool {
    POOL.get_or_init(init_pool).await
}

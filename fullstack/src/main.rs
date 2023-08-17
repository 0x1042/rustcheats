mod meta;
mod router;

use std::{env, net::SocketAddr, path::PathBuf};

use axum::{routing::get, Router, Server};
use tokio::signal;
use tower_http::{
    cors::{Any, CorsLayer},
    services::ServeDir,
};
use tracing::{info, warn};

use crate::router::meta::meta;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "DEBUG");
    }
    tracing_subscriber::fmt::init();

    let cors = CorsLayer::new().allow_origin(Any);

    let assets_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("front/build/");

    let static_files_service = ServeDir::new(assets_dir).append_index_html_on_directories(true);

    let app = Router::new()
        .route("/", get(meta))
        .fallback_service(static_files_service)
        .layer(cors);

    let addr = "0.0.0.0:3000".parse::<SocketAddr>().unwrap();

    info!("server listen at {:?}", &addr);

    Server::bind(&addr)
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

pub async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c().await.unwrap();
    };

    #[cfg(unix)]
    let terminal = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .unwrap()
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _  = ctrl_c =>{},
        _  = terminal =>{},
    }
    warn!("signal received, starting graceful shutdown");
}

use std::{env, net::SocketAddr};

use axum::{routing::get, Router, Server};
use axumex::{
    infra,
    router::{sse::sse, todo::todorouter, weather::weatherrouter},
};
use infra::signal::shutdown_signal;
use tower_http::cors::{Any, CorsLayer};
use tracing::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "DEBUG");
    }
    tracing_subscriber::fmt::init();

    let cors = CorsLayer::new().allow_origin(Any);

    let app = Router::new()
        .route(
            "/",
            get(|| async { format!("welcome, now is {}", chrono::Local::now()) }),
        )
        .nest("/todo", todorouter().await)
        .nest("/sse", sse().await)
        .nest("/weather", weatherrouter().await)
        .layer(cors);

    let addr = "0.0.0.0:3000".parse::<SocketAddr>().unwrap();

    info!("server listen at {:?}", &addr);

    Server::bind(&addr)
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

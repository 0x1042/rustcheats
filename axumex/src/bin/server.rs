use std::{env, net::SocketAddr};

use axum::{routing::get, Router};
use axumex::router::{sse::sse, todo::todorouter, weather::weatherrouter};
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

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    axum::serve(listener, app).await.unwrap();

    Ok(())
}

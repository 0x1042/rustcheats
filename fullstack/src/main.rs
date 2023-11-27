mod meta;
mod router;

use std::{convert::Infallible, env, net::SocketAddr, path::PathBuf};

use axum::{extract::Request, routing::get, Router};
use hyper::body::Incoming;
use hyper_util::{
    rt::{TokioExecutor, TokioIo},
    server,
};
use tokio::{net::TcpListener, signal};
use tower::{Service, ServiceExt};
use tower_http::services::{ServeDir, ServeFile};
use tracing::{info, warn};

use crate::router::meta::meta;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "DEBUG");
    }
    tracing_subscriber::fmt::init();

    let assets_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("front/build/");

    let static_files_service =
        ServeDir::new(assets_dir).not_found_service(ServeFile::new("assets/index.html"));

    let app = Router::new()
        .route("/", get(meta))
        .fallback_service(static_files_service);

    let addr = "0.0.0.0:3000".parse::<SocketAddr>().unwrap();

    let mut make_service = app.into_make_service_with_connect_info::<SocketAddr>();
    let listener = TcpListener::bind(addr).await.unwrap();

    info!("server listen at {:?}", &addr);

    loop {
        let (socket, remote_addr) = listener.accept().await.unwrap();

        // We don't need to call `poll_ready` because `IntoMakeServiceWithConnectInfo` is always
        // ready.
        let tower_service = unwrap_infallible(make_service.call(remote_addr).await);

        tokio::spawn(async move {
            let socket = TokioIo::new(socket);

            let hyper_service = hyper::service::service_fn(move |request: Request<Incoming>| {
                tower_service.clone().oneshot(request)
            });

            if let Err(err) = server::conn::auto::Builder::new(TokioExecutor::new())
                .serve_connection_with_upgrades(socket, hyper_service)
                .await
            {
                eprintln!("failed to serve connection: {err:#}");
            }
        });
    }

    Ok(())
}

fn unwrap_infallible<T>(result: Result<T, Infallible>) -> T {
    match result {
        Ok(value) => value,
        Err(err) => match err {},
    }
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

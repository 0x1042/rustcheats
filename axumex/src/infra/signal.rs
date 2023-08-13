use tokio::signal;
use tracing::warn;

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

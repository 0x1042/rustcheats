use tracing::info;

mod http_cli;
mod json;
mod timer_job;
mod traits;

#[tokio::main]
async fn main() {
    std::env::set_var("RUST_LOG", "INFO");
    tracing_subscriber::fmt::init();

    tokio::join!(timer_job::init(),);

    std::thread::sleep(std::time::Duration::from_secs(1));

    info!("get key {}", timer_job::get("status").await);

    std::thread::sleep(std::time::Duration::from_secs(60));
}

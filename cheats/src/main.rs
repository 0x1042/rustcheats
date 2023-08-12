mod exception;
mod http_cli;
mod json;
mod timeout;
mod timer_job;
mod traits;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "DEBUG");
    }
    tokio::join!(timer_job::init(),);
    Ok(())
}

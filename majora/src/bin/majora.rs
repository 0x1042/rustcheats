#[tokio::main]
async fn main() -> anyhow::Result<()> {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "TRACE");
    }

    tracing_subscriber::fmt::init();

    Ok(())
}

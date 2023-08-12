use tracing_subscriber::{fmt, prelude::__tracing_subscriber_SubscriberExt};

mod exception;
mod http_cli;
mod json;
mod timeout;
mod timer_job;
mod traits;

fn setup_tracing() -> anyhow::Result<()> {
    let logfile = tracing_appender::rolling::daily("./log", "app.log");

    let log_fmt = tracing_subscriber::fmt::Layer::default()
        .pretty()
        .with_writer(logfile);

    let status = tracing::subscriber::set_global_default(
        fmt::Subscriber::builder()
            .with_file(true)
            .with_line_number(true)
            .finish()
            .with(log_fmt),
    );

    match status {
        Ok(_) => Ok(()),
        Err(err) => {
            println!("{err:?}");
            Err(err.into())
        }
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    std::env::set_var("RUST_LOG", "INFO");
    // tracing_subscriber::fmt::init();
    setup_tracing()?;
    tokio::join!(timer_job::init(),);

    Ok(())
}

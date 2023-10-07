use clap::Parser;
use socks5v2::{
    core::{auth::Config, server::Server},
    Opt,
};
use tracing::debug;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "TRACE");
    }

    tracing_subscriber::fmt::init();

    let opt = Opt::parse();

    debug!("opt is {:?}", &opt);

    let addr = format!("{}:{}", &opt.addr.to_owned(), &opt.port)
        .parse::<std::net::SocketAddr>()
        .unwrap();

    let config = Config::new()
        .enable_dns_resolve()
        .timeout(std::time::Duration::from_secs(1))
        .auth_file(opt.auth_file.clone())
        .build();

    debug!("config info {:?}", &config);
    let mut server = Server::new(addr).await?;
    server.set_config(config);
    server.serve().await;

    Ok(())
}

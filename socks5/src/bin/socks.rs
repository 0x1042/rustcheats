use std::net::SocketAddr;

use clap::Parser;
use socks5::{core::server, Options};

#[tokio::main]
async fn main() {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "INFO");
    }
    tracing_subscriber::fmt::init();

    let opt = Options::parse();

    let addr_str = format!("{}:{}", opt.bind_host, opt.bind_port);

    let addr = addr_str.parse::<SocketAddr>().unwrap();

    server::start(&addr, opt.timeout).await.unwrap();
}

use std::time::Duration;

pub mod core;

#[derive(clap::Parser, Debug, Clone)]
pub struct Opt {
    /// server listen address
    #[arg(short, long, default_value = "127.0.0.1")]
    pub addr: String,

    /// server listen port
    #[arg(short, long, default_value_t = 10085)]
    pub port: u16,

    /// timeout
    #[arg(short, long, default_value = "1000", value_parser = parse_duration)]
    pub timeout: Duration,
}

fn parse_duration(arg: &str) -> Result<Duration, std::num::ParseIntError> {
    let ms = arg.parse()?;
    Ok(Duration::from_millis(ms))
}

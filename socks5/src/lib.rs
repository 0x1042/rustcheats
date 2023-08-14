use std::time::Duration;

pub mod core;

#[derive(clap::Parser)]
pub struct Options {
    /// bind host
    #[arg(long, default_value = "127.0.0.1")]
    pub bind_host: String,

    /// bind port
    #[arg(long, default_value_t = 8080)]
    pub bind_port: u16,

    /// connect timeout
    #[arg(short, long, default_value = "1000", value_parser = parse_duration)]
    pub timeout: Duration,
}

fn parse_duration(arg: &str) -> Result<Duration, std::num::ParseIntError> {
    let ms = arg.parse()?;
    Ok(Duration::from_millis(ms))
}

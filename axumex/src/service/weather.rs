use std::time::Duration;

use reqwest::Proxy;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct GeoResponse {
    pub results: Vec<LatLong>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct LatLong {
    pub latitude: f64,
    pub longitude: f64,
}

pub fn new_cli() -> reqwest::Client {
    reqwest::Client::builder()
        .pool_idle_timeout(Duration::from_secs(60))
        .pool_max_idle_per_host(32)
        .timeout(Duration::from_secs(10))
        .tcp_keepalive(Duration::from_secs(60))
        .tcp_nodelay(true)
        .connection_verbose(true)
        .proxy(Proxy::http("http://127.0.0.1:1087").unwrap())
        .build()
        .unwrap()
}

lazy_static::lazy_static! {
    static ref CLIENT: reqwest::Client = new_cli();
}

// https://www.shuttle.rs/blog/2023/09/27/rust-vs-go-comparison#a-rust-web-service
pub async fn weather_info(city: &str) -> anyhow::Result<String, Box<dyn std::error::Error>> {
    let latong = fetch_lat_long(city).await?;
    let resp = fetch(latong).await?;
    Ok(resp)
}

async fn fetch_lat_long(city: &str) -> anyhow::Result<LatLong, Box<dyn std::error::Error>> {
    let url = "https://geocoding-api.open-meteo.com/v1/search";

    let query = vec![
        ("name", city),
        ("count", "1"),
        ("language", "en"),
        ("format", "json"),
    ];

    let response = CLIENT
        .get(url)
        .query(&query)
        .send()
        .await?
        .json::<GeoResponse>()
        .await?;

    let resp = response.results.get(0).cloned();

    match resp {
        Some(llong) => Ok(llong),
        None => Err("not found".into()),
    }
}

async fn fetch(latong: LatLong) -> anyhow::Result<String, Box<dyn std::error::Error>> {
    let url = "https://api.open-meteo.com/v1/forecast";

    let query = vec![
        ("latitude", latong.latitude.to_string()),
        ("longitude", latong.longitude.to_string()),
        ("hourly", "temperature_2m".to_owned()),
    ];

    let response = CLIENT.get(url).query(&query).send().await?.text().await?;

    Ok(response)
}

#[cfg(test)]
mod tests {
    use crate::service::weather::{fetch, fetch_lat_long};

    #[tokio::test]
    async fn test_fetch_weather() {
        let latong = fetch_lat_long("Chengdu").await;
        println!("latong:{:?}", latong);

        if latong.is_ok() {
            let resp = fetch(latong.unwrap()).await;
            println!("weather resp:{:?}", resp);
        }
    }
}

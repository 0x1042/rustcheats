use axum::{extract::Path, http::StatusCode, routing::get, Router};
use tracing::info;

use crate::service::weather::weather_info;

pub async fn weatherrouter() -> Router {
    info!("init weatherrouter ");

    Router::new().route("/info/:city", get(info))
}

async fn info(Path(city): Path<String>) -> axum::response::Result<String, StatusCode> {
    let resp = weather_info(city.as_str()).await;

    match resp {
        Ok(rsp) => Ok(rsp),
        Err(_) => Err(StatusCode::NOT_FOUND),
    }
}

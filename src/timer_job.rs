use std::collections::HashMap;

use crate::http_cli;

use arc_swap::ArcSwap;
use tracing::info;

lazy_static::lazy_static! {
    static ref CLIENT: reqwest::Client = http_cli::init();

    static ref CACHE: ArcSwap<HashMap<String,String>> = ArcSwap::from_pointee(HashMap::new());
}

async fn real_get(url: &str) -> HashMap<String, String> {
    let mut dst = HashMap::with_capacity(2);

    match CLIENT.get(url).send().await {
        Ok(rsp) => {
            let mut buf = itoa::Buffer::new();
            dst.insert(
                "status".to_string(),
                buf.format(rsp.status().as_u16()).to_owned(),
            );

            match rsp.text().await {
                Ok(text) => {
                    dst.insert("body".to_string(), text);
                }
                Err(err) => {
                    tracing::error!("fetch {} with error. {}", url, err);
                }
            }
        }
        Err(err) => {
            tracing::error!("fetch {} with error. {}", url, err);
        }
    }

    dst
}

pub async fn init() {
    tokio::spawn(async move {
        loop {
            CACHE.store(std::sync::Arc::new(
                real_get("https://gocn.vip/c/3lQ6GbD5ny/s/Gd7BTB/d/z63pjQHmo3").await,
            ));
            tokio::time::sleep(std::time::Duration::from_secs(5)).await;
            let size = CACHE.load().len();
            info!("timed run... cache size {}", size);
        }
    });
}

pub async fn get(key: &str) -> String {
    let db = CACHE.load();
    db.get(key).cloned().unwrap_or_default()
}

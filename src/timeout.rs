use futures_util::SinkExt;
use std::ops::Add;
use std::time::Duration;
use tracing::{error, info};

async fn get<T>(url: T) -> anyhow::Result<String>
where
    T: reqwest::IntoUrl,
{
    let resp = reqwest::get(url).await?;
    let text = resp.text().await?;
    Ok(text)
}

pub async fn get_with_timeout<T>(url: T, timout: Duration) -> anyhow::Result<()>
where
    T: reqwest::IntoUrl + Send + 'static,
{
    let (r, w) = tokio::sync::oneshot::channel();

    tokio::spawn(async move {
        let rsp = tokio::time::timeout(timout, get(url)).await;
        match rsp {
            Ok(text) => {
                let _ = r.send(text.unwrap());
            }
            Err(err) => {
                error!("timeout error. {}", err);
                let _ = r.send("timeout error".to_string());
            }
        }
    });

    let outer = timout.add(Duration::from_millis(10));

    match tokio::time::timeout(outer, w).await {
        Ok(rsp) => {
            info!("outer success. {:?}", rsp);
        }
        Err(err) => {
            info!("outer fail. {:?}", err);
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[tokio::test]
    #[tracing_test::traced_test]
    async fn test_timeout() {
        let url = "https://www.toutiao.com";

        let timeout = Duration::from_millis(10000);

        let _ = get_with_timeout(url, timeout).await;
    }
}

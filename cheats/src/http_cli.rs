use std::time::Duration;

use reqwest::Proxy;

pub fn init() -> reqwest::Client {
    reqwest::Client::builder()
        .pool_idle_timeout(Duration::from_secs(30))
        .pool_max_idle_per_host(32)
        .timeout(Duration::from_secs(1))
        .proxy(Proxy::http("http://127.0.0.1:1087").unwrap())
        .build()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use reqwest::{Error, Response};
    use tracing::error;

    #[tokio::test]
    #[tracing_test::traced_test]
    async fn test_get() {
        let cli = crate::http_cli::init();

        let resp = cli.get("https://www.baidu.com/").send().await;

        match resp {
            Ok(text) => {
                assert_eq!(200, text.status());
                assert!(text.text().await.unwrap().len() > 0);
            }
            Err(err) => {
                tracing::error!("fetch error. {}", err);
            }
        }
    }

    #[tokio::test]
    #[tracing_test::traced_test]
    async fn test_post() {
        let cli = crate::http_cli::init();

        let resp = cli
            .post("https://www.baidu.com/")
            .body("hello world")
            .send()
            .await;

        match resp {
            Ok(text) => {
                tracing::info!("status {:?}", text.status());
            }
            Err(err) => {
                tracing::error!("fetch error. {}", err);
            }
        }
    }

    #[tokio::test]
    #[tracing_test::traced_test]
    async fn download() {
        let url =
            "https://inews.gtimg.com/om_bt/O5iwc3sJjyyn6slOb0XefgSSsoJZ5HBFbiPq8I4pdEpKsAA/1000";
        let cli = crate::http_cli::init();
        let response = cli.get(url).send().await;

        match response {
            Ok(rsp) => {
                let mut file = std::fs::File::create("image.png").unwrap();
                let mut content = std::io::Cursor::new(rsp.bytes().await.unwrap());
                std::io::copy(&mut content, &mut file).unwrap();
            }
            Err(err) => {
                error!("error: {}", err);
            }
        }
    }

    #[tokio::test]
    #[tracing_test::traced_test]
    async fn get_with_timeout() {
        let cli = crate::http_cli::init();

        let resp = cli
            .get("https://www.baidu.com/")
            .timeout(std::time::Duration::from_millis(100))
            .send()
            .await;

        match resp {
            Ok(text) => {
                assert_eq!(200, text.status());
                assert!(text.text().await.unwrap().len() > 0);
            }
            Err(err) => {
                tracing::error!("fetch error. {}", err);
            }
        }
    }
}

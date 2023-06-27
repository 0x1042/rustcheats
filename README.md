# rustcheats

- [rustcheats](#rustcheats)
- [全局变量](#全局变量)
  - [`lazy static`](#lazy-static)
  - [`once cell`](#once-cell)
- [定时任务](#定时任务)
  - [标准库](#标准库)
  - [`tokio`](#tokio)
- [`json`](#json)
  - [`marshal`](#marshal)
    - [`struct to json`](#struct-to-json)
    - [`map to json`](#map-to-json)
  - [`unmarshal`](#unmarshal)
    - [`json to struct`](#json-to-struct)
    - [`json to map`](#json-to-map)
    - [`read json file`](#read-json-file)
- [`protobuf`](#protobuf)
  - [`marshal`](#marshal-1)
  - [`thrift`](#thrift)
- [`protobuf`](#protobuf-1)
  - [`marshal`](#marshal-2)
  - [`unmarshal`](#unmarshal-1)
- [`http client`](#http-client)
  - [`长连接`](#长连接)
  - [`proxy`](#proxy)
  - [`超时控制`](#超时控制)
  - [`post`](#post)
  - [下载文件](#下载文件)
- [`http server`](#http-server)
  - [`axum`](#axum)
- [宏](#宏)
  - [声明式宏 `declarative macros`](#声明式宏-declarative-macros)
  - [过程宏 `procedural macros`](#过程宏-procedural-macros)
    - [派生宏 `#[derive]`](#派生宏-derive)
    - [类属性宏(`Attribute-like macro`)](#类属性宏attribute-like-macro)
    - [类函数宏(`Function-like macro`)](#类函数宏function-like-macro)

# 全局变量 

- `lazy static`
- `once cell`

## `lazy static`

```rust

pub fn init() -> reqwest::Client {
    reqwest::Client::builder()
        .pool_idle_timeout(Duration::from_secs(30))
        .pool_max_idle_per_host(32)
        .timeout(Duration::from_secs(1))
        .proxy(Proxy::http("http://127.0.0.1:1087").unwrap())
        .build()
        .unwrap()
}

lazy_static::lazy_static! {
    static ref CLIENT: reqwest::Client = http_cli::init();

    static ref CACHE: ArcSwap<HashMap<String,String>> = ArcSwap::from_pointee(HashMap::new());
}
```

## `once cell`

# 定时任务 

## 标准库 

## `tokio` 

```rust 
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
```

# `json`

```rust
// cargo add serde_json
// cargo add serde -F derive
#[derive(Debug, Serialize, Deserialize)]
pub struct Animal {
    pub name: String,
    pub age: u32,
}
```

## `marshal`

### `struct to json`

```rust
    #[traced_test]
    #[test]
    fn struct_to_json() {
        let tom = Animal {
            name: "tom".to_string(),
            age: 20,
        };
        let jstr = serde_json::to_string(&tom).unwrap();
        assert!(jstr.len() > 0);
    }
```
### `map to json`

```rust
    #[test]
    #[traced_test]
    fn map_to_json() {
        let mut db = HashMap::new();
        db.insert(
            "tom",
            Animal {
                name: "tom".to_owned(),
                age: 10,
            },
        );
        db.insert(
            "jerry",
            Animal {
                name: "jerry".to_owned(),
                age: 12,
            },
        );
        let jstr = serde_json::to_string(&db).unwrap();
        assert!(jstr.len() > 0);
    }
```

## `unmarshal`

### `json to struct`

```rust
    #[test]
    #[traced_test]
    fn struct_from_json() {
        let jstr = "{\"name\":\"tom\",\"age\":20}";

        let cat: Animal = serde_json::from_str(jstr).unwrap();
        assert_eq!(cat.age, 20);
    }
```
### `json to map`

```rust
    #[test]
    #[traced_test]
    fn json_to_map() {
        let jstr = "{\"name\":\"tom\",\"age\":20}";

        let db: HashMap<String, serde_json::Value> = serde_json::from_str(jstr).unwrap();

        assert_eq!(db.len(), 2);
    }
```

### `read json file`

```rust
    #[test]
    #[traced_test]
    fn map_to_json() {
        let mut db = HashMap::new();
        db.insert(
            "tom",
            Animal {
                name: "tom".to_owned(),
                age: 10,
            },
        );
        db.insert(
            "jerry",
            Animal {
                name: "jerry".to_owned(),
                age: 12,
            },
        );
        let jstr = serde_json::to_string(&db).unwrap();
        assert!(jstr.len() > 0);
    }
```

# `protobuf`

## `marshal`

## `thrift`

# `protobuf`

## `marshal`

## `unmarshal`

# `http client`

```shell
cargo add tracing-test
cargo add reqwest -F json
```

## `长连接`

```rust
pub fn init() -> reqwest::Client {
    reqwest::Client::builder()
        .pool_idle_timeout(Duration::from_secs(30))
        .pool_max_idle_per_host(32)
        .timeout(Duration::from_secs(1))
        .build()
        .unwrap()
}
```

## `proxy`

```rust
pub fn init() -> reqwest::Client {
    reqwest::Client::builder()
        .pool_idle_timeout(Duration::from_secs(30))
        .pool_max_idle_per_host(32)
        .timeout(Duration::from_secs(1))
        .proxy(Proxy::http("http://127.0.0.1:1087").unwrap())
        .build()
        .unwrap()
}
```

## `超时控制`

```rust
    #[tokio::test]
    #[tracing_test::traced_test]
    async fn get_with_timeout() {
        let cli = crate::http_cli::init();

        let resp = cli
            .get("https://www.baidu.com/")
            .timeout(Duration::from_millis(100))
            .send()
            .await;

        match resp {
            Ok(text) => {
                assert_eq!(200, text.status());
                assert!(text.text().await.unwrap().len() > 0);
            }
            Err(err) => {
                error!("fetch error. {}", err);
            }
        }
    }
```

## `post`

```rust
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
                info!("status {:?}", text.status());
            }
            Err(err) => {
                error!("fetch error. {}", err);
            }
        }
    }
```

## 下载文件 

```rust
    #[tokio::test]
    #[tracing_test::traced_test]
    async fn download() {
        let url =
            "https://inews.gtimg.com/om_bt/O5iwc3sJjyyn6slOb0XefgSSsoJZ5HBFbiPq8I4pdEpKsAA/1000";
        let cli = crate::http_cli::init();
        let response = cli.get(url).send().await.unwrap();
        let mut file = File::create("image.png").unwrap();
        let mut content = Cursor::new(response.bytes().await.unwrap());
        std::io::copy(&mut content, &mut file).unwrap();
    }
```

# `http server`

## `axum`

# 宏 

## 声明式宏 `declarative macros`

## 过程宏 `procedural macros`

### 派生宏 `#[derive]`

### 类属性宏(`Attribute-like macro`)

### 类函数宏(`Function-like macro`)
use std::sync::Arc;

pub trait Authentication: Send + Sync {
    fn authenticate(&self, username: &str, password: &str) -> bool;
}

pub struct UserPassword {
    pub username: String,
    pub password: String,
}

impl Authentication for UserPassword {
    fn authenticate(&self, username: &str, password: &str) -> bool {
        return self.username == username && self.password == password;
    }
}

pub struct NoAuth;

impl Authentication for NoAuth {
    fn authenticate(&self, _username: &str, _password: &str) -> bool {
        return true;
    }
}

#[derive(Clone)]
pub struct Config {
    pub timeout: std::time::Duration,
    pub dns_resolve: bool,
    pub auth: Arc<dyn Authentication>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            timeout: std::time::Duration::from_secs(5),
            dns_resolve: false,
            auth: Arc::new(NoAuth),
        }
    }
}

impl Config {
    pub fn new() -> ConfigBuilder {
        return ConfigBuilder {
            timeout: std::time::Duration::from_secs(5),
            dns_resolve: false,
            auth: Arc::new(NoAuth),
        };
    }
}

pub struct ConfigBuilder {
    pub timeout: std::time::Duration,
    pub dns_resolve: bool,
    pub auth: Arc<dyn Authentication>,
}

impl ConfigBuilder {
    pub fn timeout(&mut self, timeout: std::time::Duration) -> &mut ConfigBuilder {
        self.timeout = timeout;
        self
    }

    pub fn enable_dns_resolve(&mut self) -> &mut ConfigBuilder {
        self.dns_resolve = true;
        self
    }

    pub fn auth<T: Authentication + 'static>(&mut self, authentication: T) -> &mut ConfigBuilder {
        self.auth = Arc::new(authentication);
        self
    }

    pub fn build(&self) -> Config {
        Config {
            timeout: self.timeout,
            dns_resolve: self.dns_resolve,
            auth: self.auth.clone(),
        }
    }
}

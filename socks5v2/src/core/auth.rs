use std::{collections::HashMap, fs::File, io, io::BufRead, path::Path, sync::Arc};

use io::{BufReader, Lines};

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

#[derive(Clone, Debug)]
pub struct Config {
    pub timeout: std::time::Duration,
    pub dns_resolve: bool,
    pub auth_db: Arc<HashMap<String, String>>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            timeout: std::time::Duration::from_secs(5),
            dns_resolve: false,
            auth_db: Default::default(),
        }
    }
}

impl Config {
    pub fn new() -> ConfigBuilder {
        return ConfigBuilder {
            timeout: std::time::Duration::from_secs(5),
            dns_resolve: false,
            auth_db: Default::default(),
        };
    }
}

pub struct ConfigBuilder {
    pub timeout: std::time::Duration,
    pub dns_resolve: bool,
    pub auth_db: Arc<HashMap<String, String>>,
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

    pub fn auth_file(&mut self, db_file: Option<String>) -> &mut ConfigBuilder {
        if db_file.is_some() {
            let db = parse_auth_file(db_file.unwrap()).unwrap();
            self.auth_db = Arc::new(db);
        }

        self
    }

    pub fn build(&self) -> Config {
        Config {
            timeout: self.timeout,
            dns_resolve: self.dns_resolve,
            auth_db: self.auth_db.clone(),
        }
    }
}

fn read_lines<P>(filename: P) -> anyhow::Result<Lines<BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}

fn parse_auth_file<P>(filename: P) -> anyhow::Result<HashMap<String, String>>
where
    P: AsRef<Path>,
{
    let mut db = HashMap::new();
    if let Ok(lines) = read_lines(filename) {
        for line in lines {
            if line.is_err() {
                continue;
            }
            let rl = line.unwrap();
            let auth_info = rl.split(":").collect::<Vec<_>>();

            if auth_info.len() != 2 {
                continue;
            }
            db.insert(auth_info[0].to_owned(), auth_info[1].to_owned());
        }
    }
    Ok(db)
}

#[cfg(test)]
mod tests {

    use crate::core::auth::parse_auth_file;

    #[test]
    fn test_parse() {
        let cur_dir = std::env::current_dir().unwrap();
        println!("current_dir {:?}", cur_dir);
        let db = parse_auth_file(cur_dir.join("../testdata/auth.db")).unwrap();
        println!("db {:?}", db);
    }
}

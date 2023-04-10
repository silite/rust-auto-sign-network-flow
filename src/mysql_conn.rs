use lazy_static::lazy_static;
use serde::Deserialize;
use std::fs::File;
use std::io::Read;
use std::sync::Mutex;
use std::{error::Error, path::Path};
use toml::from_str;

use mysql::{Pool, PooledConn};

#[derive(Debug, Default, Deserialize)]
struct Config {
    username: String,
    password: String,
    host: String,
    port: String,
    database: String,
}

impl Config {
    pub fn set_default_file<T>(&mut self, path: T) -> Result<(), Box<dyn Error>>
    where
        // https://rusty-ferris.pages.dev/blog/asref-vs-borrow-trait/
        T: AsRef<Path>,
    {
        let mut file = File::open(path)?;
        let mut config_string = String::new();
        file.read_to_string(&mut config_string)?;
        from_str(&config_string).map(|config: Config| {
            self.username = config.username;
            self.password = config.password;
            self.host = config.host;
            self.port = config.port;
            self.database = config.database;
        })?;
        Ok(())
    }
}

lazy_static! {
    pub static ref CONN: Mutex<PooledConn> = {
        let mut config = Config::default();
        config.set_default_file("config.toml").unwrap();
        let url = format!(
            "mysql://{}:{}@{}:{}/{}",
            config.username, config.password, config.host, config.port, config.database
        );
        let pool = Pool::new(url.as_str()).unwrap();
        Mutex::new(pool.get_conn().unwrap())
    };
}

#[test]
fn feature() -> Result<(), Box<dyn Error>> {
    Ok(())
}

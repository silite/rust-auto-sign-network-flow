use lazy_static::lazy_static;
use mysql::prelude::Queryable;
use serde::Deserialize;
use std::error::Error;
use std::sync::Mutex;

use mysql::{params, Pool, PooledConn};

use crate::read_config::read_config;

#[derive(Debug, Default, Deserialize)]
struct Config {
    username: String,
    password: String,
    host: String,
    port: String,
    database: String,
}

impl Config {
    pub fn set_default_file(&mut self) -> Result<(), Box<dyn Error>> {
        let config: Config = read_config("config.toml")?;
        self.username = config.username;
        self.password = config.password;
        self.host = config.host;
        self.port = config.port;
        self.database = config.database;
        Ok(())
    }
}

lazy_static! {
    pub static ref CONN: Mutex<PooledConn> = {
        let mut config = Config::default();
        config.set_default_file().unwrap();
        let url = format!(
            "mysql://{}:{}@{}:{}/{}",
            config.username, config.password, config.host, config.port, config.database
        );
        let pool = Pool::new(url.as_str()).unwrap();
        Mutex::new(pool.get_conn().unwrap())
    };
}

pub fn insert_log(log: &str) -> Result<(), Box<dyn Error>> {
    CONN.lock().unwrap().exec_batch(
        r"INSERT INTO check_in (check_in_res) VALUES (:check_in_res)",
        vec![params! {
             "check_in_res" => log
        }],
    )?;
    Ok(())
}

#[test]
fn feature() -> Result<(), Box<dyn Error>> {
    Ok(())
}

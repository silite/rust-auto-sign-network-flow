pub mod check_in;
pub mod email;
pub mod login;
pub mod mysql_conn;
pub mod read_config;

use ::mysql::prelude::Queryable;
use lazy_static::lazy_static;
use mysql_conn::CONN;
use reqwest::Client;
use std::error::Error;
use tokio::sync::Mutex;

pub fn init() -> Result<(), Box<dyn Error>> {
    CONN.lock().unwrap().query_drop(
        r"CREATE TABLE IF NOT EXISTS check_in (
                id INT(11) AUTO_INCREMENT,
                check_in_res VARCHAR(255) NOT NULL,
                PRIMARY KEY(id)
        );",
    )?;
    Ok(())
}

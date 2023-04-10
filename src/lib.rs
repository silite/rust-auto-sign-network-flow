pub mod check_in;
pub mod login;
pub mod mysql_conn;

use ::mysql::prelude::Queryable;
use lazy_static::lazy_static;
use mysql_conn::CONN;
use reqwest::Client;
use std::{error::Error, sync::Mutex};

lazy_static! {
    pub static ref CLIENT: Mutex<Client> = {
        let proxy = reqwest::Proxy::https("http://127.0.0.1:7890");
        Mutex::new(
            reqwest::Client::builder()
                .proxy(proxy.unwrap())
                .cookie_store(true)
                .build()
                .unwrap(),
        )
    };
}

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

pub mod check_in;
pub mod login;

use lazy_static::lazy_static;
use reqwest::Client;
use tokio::sync::Mutex;

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

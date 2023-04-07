#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use lazy_static::lazy_static;
use reqwest::{Client, Response};
use serde::Deserialize;
use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader, Lines},
};
use tokio::sync::Mutex;

#[derive(Deserialize, Debug)]
struct LoginInResp {
    msg: String,
    ret: i32,
}

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

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    login().await?;
    check_in().await?;
    Ok(())
}

async fn login() -> Result<(), Box<dyn Error>> {
    let pwd = get_pwd();
    let params = [("email", "1244993561@qq.com"), ("passwd", pwd.as_str())];
    let res = CLIENT
        .lock()
        .await
        .post("https://xn--gmq396grzd.com/auth/login")
        .form(&params)
        .send()
        .await?;

    async fn json_parse(res: Response) -> Result<(), Box<dyn Error>> {
        let res_response = res.json::<LoginInResp>().await?;
        println!("{:?}", res_response);
        if res_response.ret != 1 {
            println!("登录失败");
        }
        return Ok(());
    }

    json_parse(res).await?;
    Ok(())
}

#[derive(Deserialize, Debug)]
struct CheckInResp {
    msg: String,
    ret: i32,
}
async fn check_in() -> Result<(), Box<dyn Error>> {
    let res = CLIENT
        .lock()
        .await
        .post("https://xn--gmq396grzd.com/user/checkin")
        .send()
        .await?
        .json::<CheckInResp>()
        .await;
    println!("{:?}", res);
    Ok(())
}

fn get_pwd() -> String {
    let file = File::open("password");
    let iter = BufReader::new(file.unwrap()).lines();
    iter.into_iter().nth(0).unwrap().unwrap()
}

#[test]
fn feature() {
    let res = get_pwd();
    println!("{}", res);
}

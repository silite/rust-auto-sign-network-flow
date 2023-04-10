use crate::{mysql_conn::CONN, CLIENT};
use mysql::{params, prelude::Queryable};
use reqwest::Response;
use serde::Deserialize;
use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Deserialize, Debug)]
struct LoginInResp {
    msg: String,
    ret: i32,
}
pub async fn login() -> Result<(), Box<dyn Error>> {
    let pwd = get_pwd();
    let params = [("email", "1244993561@qq.com"), ("passwd", pwd.as_str())];
    let res = CLIENT
        .lock()
        .unwrap()
        .post("https://xn--gmq396grzd.com/auth/login")
        .form(&params)
        .send()
        .await?;

    async fn json_parse(res: Response) -> Result<(), Box<dyn Error>> {
        let res_response = res.json::<LoginInResp>().await?;
        println!("{:?}", res_response);
        if res_response.ret != 1 {
            CONN.lock().unwrap().exec_batch(
                r"INSERT INTO check_in (check_in_res) VALUES (:check_in_res)",
                vec![params! {
                     "check_in_res" => res_response.msg,
                }],
            )?;
            println!("登录失败");
            return Err("登录失败".into());
        }
        return Ok(());
    }

    json_parse(res).await?;
    Ok(())
}

fn get_pwd() -> String {
    let file = File::open("password");
    let iter = BufReader::new(file.unwrap()).lines();
    iter.into_iter().nth(0).unwrap().unwrap()
}

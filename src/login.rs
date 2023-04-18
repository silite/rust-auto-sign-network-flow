use crate::{email::send_email, mysql_conn::insert_log, read_config::read_config};
use reqwest::{Client, Response};
use serde::Deserialize;
use std::error::Error;

#[derive(Deserialize, Debug)]
struct LoginInResp {
    msg: String,
    ret: i32,
}
#[derive(Deserialize, Debug)]
struct LoginIdentity {
    login_pwd: String,
}
pub async fn login() -> Result<Client, Box<dyn Error>> {
    let login_identity: LoginIdentity = read_config("config.toml")?;
    let params = [
        ("email", "1244993561@qq.com"),
        ("passwd", login_identity.login_pwd.as_str()),
    ];
    let proxy = reqwest::Proxy::https("http://127.0.0.1:7890");
    let client = reqwest::Client::builder()
        .proxy(proxy.unwrap())
        .cookie_store(true)
        .build()
        .unwrap();
    let res = client
        .post("https://xn--gmq396grzd.com/auth/login")
        .form(&params)
        .send()
        .await?;

    async fn json_parse(res: Response) -> Result<(), Box<dyn Error>> {
        let res_response = match res.json::<LoginInResp>().await {
            Ok(res) => res,
            Err(_) => LoginInResp {
                msg: "".to_string(),
                ret: 1,
            },
        };
        println!("{:?}", res_response);
        if res_response.ret != 1 {
            send_email("登录失败", res_response.msg.as_str())?;
            insert_log(res_response.msg.as_str())?;
            return Err("登录失败".into());
        }
        return Ok(());
    }

    json_parse(res).await?;
    Ok(client)
}

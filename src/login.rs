use crate::{email::send_email, mysql_conn::insert_log, read_config::read_config, CLIENT};
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
#[derive(Deserialize, Debug)]
struct LoginIdentity {
    login_pwd: String,
}
pub async fn login() -> Result<(), Box<dyn Error>> {
    let login_identity: LoginIdentity = read_config("config.toml")?;
    let params = [
        ("email", "1244993561@qq.com"),
        ("passwd", login_identity.login_pwd.as_str()),
    ];
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
            send_email("登录失败", res_response.msg.as_str())?;
            insert_log(res_response.msg.as_str())?;
            return Err("登录失败".into());
        }
        return Ok(());
    }

    json_parse(res).await?;
    Ok(())
}

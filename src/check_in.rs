use std::error::Error;

use mysql::serde_json::from_str;
use serde::Deserialize;

use crate::{email::send_email, mysql_conn::insert_log, CLIENT};

#[derive(Deserialize, Debug)]
struct CheckInResp {
    msg: String,
    ret: i32,
}
pub async fn check_in() -> Result<(), Box<dyn Error>> {
    let res = CLIENT
        .lock()
        .await
        .post("https://xn--gmq396grzd.com/user/checkin")
        .send()
        .await?
        .text()
        .await?
        .replace("\"", "\\\"");

    let result: String = from_str(&format!("\"{}\"", res))?;
    send_email("签到结果", result.as_str())?;
    insert_log(result.as_str())?;
    Ok(())
}

#[test]
fn feature() -> Result<(), Box<dyn Error>> {
    let t = r#"\"\u60a8\""#;
    println!("{}", t);
    let res: String = from_str(&format!("\"{}\"", t))?;
    println!("{}", res);
    Ok(())
}

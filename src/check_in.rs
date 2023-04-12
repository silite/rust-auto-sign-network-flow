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
        .await?;

    let res: String = from_str(&res)?;
    send_email("签到结果", res.as_str())?;
    insert_log(res.as_str())?;
    Ok(())
}

#[test]
fn feature() {
    let ch = '汉' as i32;
    let ch_unicode = format!("{:X}", ch);
    println!("{}", ch);
    println!("{}", ch_unicode);
}

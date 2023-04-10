use std::error::Error;

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
        .unwrap()
        .post("https://xn--gmq396grzd.com/user/checkin")
        .send()
        .await?
        .json::<CheckInResp>()
        .await?;

    send_email("签到结果", res.msg.as_str())?;
    insert_log(res.msg.as_str())?;
    Ok(())
}

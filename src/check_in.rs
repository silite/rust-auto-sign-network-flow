use std::error::Error;

use serde::Deserialize;

use crate::{mysql_conn::insert_log, CLIENT};

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

    insert_log(res.msg.as_str())?;
    Ok(())
}

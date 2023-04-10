use std::error::Error;

use mysql::{params, prelude::Queryable};
use serde::Deserialize;

use crate::{mysql_conn::CONN, CLIENT};

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
    CONN.lock().unwrap().exec_batch(
        r"INSERT INTO check_in (check_in_res) VALUES (:check_in_res)",
        vec![params! {
             "check_in_res" => res.msg
        }],
    )?;
    Ok(())
}

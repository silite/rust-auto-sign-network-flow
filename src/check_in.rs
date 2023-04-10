use std::error::Error;

use serde::Deserialize;

use crate::CLIENT;

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
        .json::<CheckInResp>()
        .await;
    println!("{:?}", res);
    Ok(())
}

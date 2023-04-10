#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use rust_auto_sign_network_flow::{check_in::check_in, init, login::login};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    init()?;
    login().await?;
    check_in().await?;
    Ok(())
}

#[test]
fn feature() {}

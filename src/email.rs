use std::error::Error;

use lettre::{
    message::header::ContentType, transport::smtp::authentication::Credentials, Message,
    SmtpTransport, Transport,
};
use serde::Deserialize;

use crate::{mysql_conn::insert_log, read_config::read_config};

#[derive(Deserialize, Debug)]
struct EmailIdentity {
    email_pwd: String,
}

pub fn send_email(subject: &str, body: &str) -> Result<(), Box<dyn Error>> {
    let email = Message::builder()
        .from("Log <1244993561@qq.com>".parse().unwrap())
        .to("Silite <lemnsilite@gmail.com>".parse().unwrap())
        .subject(subject)
        .header(ContentType::TEXT_PLAIN)
        .body(String::from(body))
        .unwrap();

    let email_identity: EmailIdentity = read_config("config.toml")?;

    let creds = Credentials::new(
        "1244993561@qq.com".to_owned(),
        email_identity.email_pwd.to_owned(),
    );

    // Open a remote connection to gmail
    let mailer = SmtpTransport::relay("smtp.qq.com")
        .unwrap()
        .credentials(creds)
        .build();

    match mailer.send(&email) {
        Ok(_) => println!("Email sent successfully!"),
        Err(e) => {
            insert_log(format!("Could not send email: {e:?}").as_str())?;
        }
    }
    Ok(())
}

#[test]
fn feature() -> Result<(), Box<dyn Error>> {
    send_email("success", "check in success")?;
    Ok(())
}

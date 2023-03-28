#![allow(unused_imports)]
use twilio::TwilioClient;
use twilio::model::*;
#[tokio::main]
async fn main() {
    let client = TwilioClient::from_env();
    let account_sid = "your account sid";
    let response = client
        .list_short_code(account_sid)
        .friendly_name("your friendly name")
        .page(1)
        .page_size(1)
        .page_token("your page token")
        .short_code("your short code")
        .await
        .unwrap();
    println!("{:#?}", response);
}
#![allow(unused_imports)]
use twilio::TwilioClient;
use twilio::model::*;
#[tokio::main]
async fn main() {
    let client = TwilioClient::from_env();
    let account_sid = "your account sid";
    let connect_app_sid = "your connect app sid";
    let response = client
        .fetch_authorized_connect_app(account_sid, connect_app_sid)
        .await
        .unwrap();
    println!("{:#?}", response);
}
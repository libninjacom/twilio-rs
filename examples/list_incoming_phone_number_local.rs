#![allow(unused_imports)]
use twilio::TwilioClient;
use twilio::model::*;
#[tokio::main]
async fn main() {
    let client = TwilioClient::from_env();
    let account_sid = "your account sid";
    let response = client
        .list_incoming_phone_number_local(account_sid)
        .beta(true)
        .friendly_name("your friendly name")
        .origin("your origin")
        .page(1)
        .page_size(1)
        .page_token("your page token")
        .phone_number("your phone number")
        .await
        .unwrap();
    println!("{:#?}", response);
}
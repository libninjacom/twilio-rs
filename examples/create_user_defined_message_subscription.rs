#![allow(unused_imports)]
use twilio::TwilioClient;
use twilio::model::*;
#[tokio::main]
async fn main() {
    let client = TwilioClient::from_env();
    let account_sid = "your account sid";
    let call_sid = "your call sid";
    let response = client
        .create_user_defined_message_subscription(account_sid, call_sid)
        .await
        .unwrap();
    println!("{:#?}", response);
}
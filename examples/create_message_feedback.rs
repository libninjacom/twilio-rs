#![allow(unused_imports)]
use twilio::TwilioClient;
use twilio::model::*;
#[tokio::main]
async fn main() {
    let client = TwilioClient::from_env();
    let account_sid = "your account sid";
    let message_sid = "your message sid";
    let response = client
        .create_message_feedback(account_sid, message_sid)
        .await
        .unwrap();
    println!("{:#?}", response);
}
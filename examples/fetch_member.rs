#![allow(unused_imports)]
use twilio::TwilioClient;
use twilio::model::*;
#[tokio::main]
async fn main() {
    let client = TwilioClient::from_env();
    let account_sid = "your account sid";
    let call_sid = "your call sid";
    let queue_sid = "your queue sid";
    let response = client.fetch_member(account_sid, call_sid, queue_sid).await.unwrap();
    println!("{:#?}", response);
}
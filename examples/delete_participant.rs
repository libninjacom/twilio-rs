#![allow(unused_imports)]
use twilio::TwilioClient;
use twilio::model::*;
#[tokio::main]
async fn main() {
    let client = TwilioClient::from_env();
    let account_sid = "your account sid";
    let call_sid = "your call sid";
    let conference_sid = "your conference sid";
    let response = client
        .delete_participant(account_sid, call_sid, conference_sid)
        .await
        .unwrap();
    println!("{:#?}", response);
}
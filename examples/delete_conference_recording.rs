#![allow(unused_imports)]
use twilio::TwilioClient;
use twilio::model::*;
#[tokio::main]
async fn main() {
    let client = TwilioClient::from_env();
    let account_sid = "your account sid";
    let conference_sid = "your conference sid";
    let sid = "your sid";
    let response = client
        .delete_conference_recording(account_sid, conference_sid, sid)
        .await
        .unwrap();
    println!("{:#?}", response);
}
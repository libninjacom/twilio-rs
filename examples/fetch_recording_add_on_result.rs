#![allow(unused_imports)]
use twilio::TwilioClient;
use twilio::model::*;
#[tokio::main]
async fn main() {
    let client = TwilioClient::from_env();
    let account_sid = "your account sid";
    let reference_sid = "your reference sid";
    let sid = "your sid";
    let response = client
        .fetch_recording_add_on_result(account_sid, reference_sid, sid)
        .await
        .unwrap();
    println!("{:#?}", response);
}
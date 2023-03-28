#![allow(unused_imports)]
use twilio::TwilioClient;
use twilio::model::*;
#[tokio::main]
async fn main() {
    let client = TwilioClient::from_env();
    let account_sid = "your account sid";
    let conference_sid = "your conference sid";
    let response = client
        .list_conference_recording(account_sid, conference_sid)
        .date_created(chrono::Utc::now().date_naive())
        .date_created_gt(chrono::Utc::now().date_naive())
        .date_created_lt(chrono::Utc::now().date_naive())
        .page(1)
        .page_size(1)
        .page_token("your page token")
        .await
        .unwrap();
    println!("{:#?}", response);
}
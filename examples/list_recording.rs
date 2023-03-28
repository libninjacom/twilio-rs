#![allow(unused_imports)]
use twilio::TwilioClient;
use twilio::model::*;
#[tokio::main]
async fn main() {
    let client = TwilioClient::from_env();
    let account_sid = "your account sid";
    let response = client
        .list_recording(account_sid)
        .call_sid("your call sid")
        .conference_sid("your conference sid")
        .date_created(chrono::Utc::now())
        .date_created_gt(chrono::Utc::now())
        .date_created_lt(chrono::Utc::now())
        .include_soft_deleted(true)
        .page(1)
        .page_size(1)
        .page_token("your page token")
        .await
        .unwrap();
    println!("{:#?}", response);
}
#![allow(unused_imports)]
use twilio::TwilioClient;
use twilio::model::*;
#[tokio::main]
async fn main() {
    let client = TwilioClient::from_env();
    let account_sid = "your account sid";
    let response = client
        .list_conference(account_sid)
        .date_created(chrono::Utc::now().date_naive())
        .date_created_gt(chrono::Utc::now().date_naive())
        .date_created_lt(chrono::Utc::now().date_naive())
        .date_updated(chrono::Utc::now().date_naive())
        .date_updated_gt(chrono::Utc::now().date_naive())
        .date_updated_lt(chrono::Utc::now().date_naive())
        .friendly_name("your friendly name")
        .page(1)
        .page_size(1)
        .page_token("your page token")
        .status("your status")
        .await
        .unwrap();
    println!("{:#?}", response);
}
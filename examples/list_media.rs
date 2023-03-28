#![allow(unused_imports)]
use twilio::TwilioClient;
use twilio::model::*;
#[tokio::main]
async fn main() {
    let client = TwilioClient::from_env();
    let account_sid = "your account sid";
    let message_sid = "your message sid";
    let response = client
        .list_media(account_sid, message_sid)
        .date_created(chrono::Utc::now())
        .date_created_gt(chrono::Utc::now())
        .date_created_lt(chrono::Utc::now())
        .page(1)
        .page_size(1)
        .page_token("your page token")
        .await
        .unwrap();
    println!("{:#?}", response);
}
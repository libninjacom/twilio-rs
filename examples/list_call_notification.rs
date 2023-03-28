#![allow(unused_imports)]
use twilio::TwilioClient;
use twilio::model::*;
#[tokio::main]
async fn main() {
    let client = TwilioClient::from_env();
    let account_sid = "your account sid";
    let call_sid = "your call sid";
    let response = client
        .list_call_notification(account_sid, call_sid)
        .log(1)
        .message_date(chrono::Utc::now().date_naive())
        .message_date_gt(chrono::Utc::now().date_naive())
        .message_date_lt(chrono::Utc::now().date_naive())
        .page(1)
        .page_size(1)
        .page_token("your page token")
        .await
        .unwrap();
    println!("{:#?}", response);
}
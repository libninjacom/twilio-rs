#![allow(unused_imports)]
use twilio::TwilioClient;
use twilio::model::*;
#[tokio::main]
async fn main() {
    let client = TwilioClient::from_env();
    let account_sid = "your account sid";
    let message_date_gt = chrono::Utc::now().date();
    let message_date_lt = chrono::Utc::now().date();
    let response = client
        .list_notification(account_sid, message_date_gt, message_date_lt)
        .log(1)
        .message_date(chrono::Utc::now().date())
        .page(1)
        .page_size(1)
        .page_token("your page token")
        .await
        .unwrap();
    println!("{:#?}", response);
}
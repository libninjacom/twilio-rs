#![allow(unused_imports)]
use twilio::TwilioClient;
use twilio::model::*;
use twilio::request::ListCallNotificationRequired;
#[tokio::main]
async fn main() {
    let client = TwilioClient::from_env();
    let args = ListCallNotificationRequired {
        account_sid: "your account sid",
        call_sid: "your call sid",
        message_date_gt: chrono::Utc::now().date(),
        message_date_lt: chrono::Utc::now().date(),
    };
    let response = client
        .list_call_notification(args)
        .log(1)
        .message_date(chrono::Utc::now().date())
        .page(1)
        .page_size(1)
        .page_token("your page token")
        .await
        .unwrap();
    println!("{:#?}", response);
}
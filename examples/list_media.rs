#![allow(unused_imports)]
use twilio::TwilioClient;
use twilio::model::*;
use twilio::request::ListMediaRequired;
#[tokio::main]
async fn main() {
    let client = TwilioClient::from_env();
    let args = ListMediaRequired {
        account_sid: "your account sid",
        date_created_gt: chrono::Utc::now(),
        date_created_lt: chrono::Utc::now(),
        message_sid: "your message sid",
    };
    let response = client
        .list_media(args)
        .date_created(chrono::Utc::now())
        .page(1)
        .page_size(1)
        .page_token("your page token")
        .await
        .unwrap();
    println!("{:#?}", response);
}
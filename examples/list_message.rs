#![allow(unused_imports)]
use twilio::TwilioClient;
use twilio::model::*;
#[tokio::main]
async fn main() {
    let client = TwilioClient::from_env();
    let account_sid = "your account sid";
    let date_sent_gt = chrono::Utc::now();
    let date_sent_lt = chrono::Utc::now();
    let response = client
        .list_message(account_sid, date_sent_gt, date_sent_lt)
        .date_sent(chrono::Utc::now())
        .from("your from")
        .page(1)
        .page_size(1)
        .page_token("your page token")
        .to("your to")
        .await
        .unwrap();
    println!("{:#?}", response);
}
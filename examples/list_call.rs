#![allow(unused_imports)]
use twilio::TwilioClient;
use twilio::model::*;
#[tokio::main]
async fn main() {
    let client = TwilioClient::from_env();
    let account_sid = "your account sid";
    let response = client
        .list_call(account_sid)
        .end_time(chrono::Utc::now())
        .end_time_gt(chrono::Utc::now())
        .end_time_lt(chrono::Utc::now())
        .from("your from")
        .page(1)
        .page_size(1)
        .page_token("your page token")
        .parent_call_sid("your parent call sid")
        .start_time(chrono::Utc::now())
        .start_time_gt(chrono::Utc::now())
        .start_time_lt(chrono::Utc::now())
        .status("your status")
        .to("your to")
        .await
        .unwrap();
    println!("{:#?}", response);
}
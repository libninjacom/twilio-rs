#![allow(unused_imports)]
use twilio::TwilioClient;
use twilio::model::*;
#[tokio::main]
async fn main() {
    let client = TwilioClient::from_env();
    let account_sid = "your account sid";
    let response = client
        .list_usage_record_today(account_sid)
        .category("your category")
        .end_date(chrono::Utc::now().date_naive())
        .include_subaccounts(true)
        .page(1)
        .page_size(1)
        .page_token("your page token")
        .start_date(chrono::Utc::now().date_naive())
        .await
        .unwrap();
    println!("{:#?}", response);
}
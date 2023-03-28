#![allow(unused_imports)]
use twilio::TwilioClient;
use twilio::model::*;
use twilio::request::ListConferenceRequired;
#[tokio::main]
async fn main() {
    let client = TwilioClient::from_env();
    let args = ListConferenceRequired {
        account_sid: "your account sid",
        date_created_gt: chrono::Utc::now().date(),
        date_created_lt: chrono::Utc::now().date(),
        date_updated_gt: chrono::Utc::now().date(),
        date_updated_lt: chrono::Utc::now().date(),
    };
    let response = client
        .list_conference(args)
        .date_created(chrono::Utc::now().date())
        .date_updated(chrono::Utc::now().date())
        .friendly_name("your friendly name")
        .page(1)
        .page_size(1)
        .page_token("your page token")
        .status("your status")
        .await
        .unwrap();
    println!("{:#?}", response);
}
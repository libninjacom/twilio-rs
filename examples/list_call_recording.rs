#![allow(unused_imports)]
use twilio::TwilioClient;
use twilio::model::*;
use twilio::request::ListCallRecordingRequired;
#[tokio::main]
async fn main() {
    let client = TwilioClient::from_env();
    let args = ListCallRecordingRequired {
        account_sid: "your account sid",
        call_sid: "your call sid",
        date_created_gt: chrono::Utc::now().date(),
        date_created_lt: chrono::Utc::now().date(),
    };
    let response = client
        .list_call_recording(args)
        .date_created(chrono::Utc::now().date())
        .page(1)
        .page_size(1)
        .page_token("your page token")
        .await
        .unwrap();
    println!("{:#?}", response);
}
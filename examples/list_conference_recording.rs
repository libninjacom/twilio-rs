#![allow(unused_imports)]
use twilio::TwilioClient;
use twilio::model::*;
use twilio::request::ListConferenceRecordingRequired;
#[tokio::main]
async fn main() {
    let client = TwilioClient::from_env();
    let args = ListConferenceRecordingRequired {
        account_sid: "your account sid",
        conference_sid: "your conference sid",
        date_created_gt: chrono::Utc::now().date(),
        date_created_lt: chrono::Utc::now().date(),
    };
    let response = client
        .list_conference_recording(args)
        .date_created(chrono::Utc::now().date())
        .page(1)
        .page_size(1)
        .page_token("your page token")
        .await
        .unwrap();
    println!("{:#?}", response);
}
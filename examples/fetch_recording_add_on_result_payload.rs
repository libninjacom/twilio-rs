#![allow(unused_imports)]
use twilio::TwilioClient;
use twilio::model::*;
use twilio::request::FetchRecordingAddOnResultPayloadRequired;
#[tokio::main]
async fn main() {
    let client = TwilioClient::from_env();
    let args = FetchRecordingAddOnResultPayloadRequired {
        account_sid: "your account sid",
        add_on_result_sid: "your add on result sid",
        reference_sid: "your reference sid",
        sid: "your sid",
    };
    let response = client.fetch_recording_add_on_result_payload(args).await.unwrap();
    println!("{:#?}", response);
}
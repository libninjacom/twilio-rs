#![allow(unused_imports)]
use twilio::TwilioClient;
use twilio::model::*;
use twilio::request::DeleteRecordingAddOnResultPayloadRequired;
#[tokio::main]
async fn main() {
    let client = TwilioClient::from_env();
    let args = DeleteRecordingAddOnResultPayloadRequired {
        account_sid: "your account sid",
        add_on_result_sid: "your add on result sid",
        reference_sid: "your reference sid",
        sid: "your sid",
    };
    let response = client.delete_recording_add_on_result_payload(args).await.unwrap();
    println!("{:#?}", response);
}
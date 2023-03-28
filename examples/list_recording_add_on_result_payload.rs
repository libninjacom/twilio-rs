#![allow(unused_imports)]
use twilio::TwilioClient;
use twilio::model::*;
#[tokio::main]
async fn main() {
    let client = TwilioClient::from_env();
    let account_sid = "your account sid";
    let add_on_result_sid = "your add on result sid";
    let reference_sid = "your reference sid";
    let response = client
        .list_recording_add_on_result_payload(
            account_sid,
            add_on_result_sid,
            reference_sid,
        )
        .page(1)
        .page_size(1)
        .page_token("your page token")
        .await
        .unwrap();
    println!("{:#?}", response);
}
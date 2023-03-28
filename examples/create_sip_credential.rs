#![allow(unused_imports)]
use twilio::TwilioClient;
use twilio::model::*;
#[tokio::main]
async fn main() {
    let client = TwilioClient::from_env();
    let account_sid = "your account sid";
    let credential_list_sid = "your credential list sid";
    let response = client
        .create_sip_credential(account_sid, credential_list_sid)
        .await
        .unwrap();
    println!("{:#?}", response);
}
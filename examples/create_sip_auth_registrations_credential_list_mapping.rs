#![allow(unused_imports)]
use twilio::TwilioClient;
use twilio::model::*;
#[tokio::main]
async fn main() {
    let client = TwilioClient::from_env();
    let account_sid = "your account sid";
    let domain_sid = "your domain sid";
    let response = client
        .create_sip_auth_registrations_credential_list_mapping(account_sid, domain_sid)
        .await
        .unwrap();
    println!("{:#?}", response);
}
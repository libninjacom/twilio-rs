#![allow(unused_imports)]
use twilio::TwilioClient;
use twilio::model::*;
#[tokio::main]
async fn main() {
    let client = TwilioClient::from_env();
    let account_sid = "your account sid";
    let resource_sid = "your resource sid";
    let response = client
        .create_incoming_phone_number_assigned_add_on(account_sid, resource_sid)
        .await
        .unwrap();
    println!("{:#?}", response);
}
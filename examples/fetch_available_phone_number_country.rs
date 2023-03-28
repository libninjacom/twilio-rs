#![allow(unused_imports)]
use twilio::TwilioClient;
use twilio::model::*;
#[tokio::main]
async fn main() {
    let client = TwilioClient::from_env();
    let account_sid = "your account sid";
    let country_code = "your country code";
    let response = client
        .fetch_available_phone_number_country(account_sid, country_code)
        .await
        .unwrap();
    println!("{:#?}", response);
}
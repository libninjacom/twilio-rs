#![allow(unused_imports)]
use twilio::TwilioClient;
use twilio::model::*;
#[tokio::main]
async fn main() {
    let client = TwilioClient::from_env();
    let response = client
        .list_account()
        .friendly_name("your friendly name")
        .page(1)
        .page_size(1)
        .page_token("your page token")
        .status("your status")
        .await
        .unwrap();
    println!("{:#?}", response);
}
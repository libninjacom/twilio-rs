#![allow(unused_imports)]
use twilio::TwilioClient;
use twilio::model::*;
#[tokio::main]
async fn main() {
    let client = TwilioClient::from_env();
    let sid = "your sid";
    let response = client.fetch_account(sid).await.unwrap();
    println!("{:#?}", response);
}
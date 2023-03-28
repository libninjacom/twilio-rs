#![allow(unused_imports)]
use twilio::TwilioClient;
use twilio::model::*;
#[tokio::main]
async fn main() {
    let client = TwilioClient::from_env();
    let account_sid = "your account sid";
    let ip_access_control_list_sid = "your ip access control list sid";
    let response = client
        .create_sip_ip_address(account_sid, ip_access_control_list_sid)
        .await
        .unwrap();
    println!("{:#?}", response);
}
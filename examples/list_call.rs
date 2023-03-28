#![allow(unused_imports)]
use twilio::TwilioClient;
use twilio::model::*;
use twilio::request::ListCallRequired;
#[tokio::main]
async fn main() {
    let client = TwilioClient::from_env();
    let args = ListCallRequired {
        account_sid: "your account sid",
        end_time_gt: chrono::Utc::now(),
        end_time_lt: chrono::Utc::now(),
        start_time_gt: chrono::Utc::now(),
        start_time_lt: chrono::Utc::now(),
    };
    let response = client
        .list_call(args)
        .end_time(chrono::Utc::now())
        .from("your from")
        .page(1)
        .page_size(1)
        .page_token("your page token")
        .parent_call_sid("your parent call sid")
        .start_time(chrono::Utc::now())
        .status("your status")
        .to("your to")
        .await
        .unwrap();
    println!("{:#?}", response);
}
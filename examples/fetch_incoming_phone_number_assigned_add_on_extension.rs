#![allow(unused_imports)]
use twilio::TwilioClient;
use twilio::model::*;
use twilio::request::FetchIncomingPhoneNumberAssignedAddOnExtensionRequired;
#[tokio::main]
async fn main() {
    let client = TwilioClient::from_env();
    let args = FetchIncomingPhoneNumberAssignedAddOnExtensionRequired {
        account_sid: "your account sid",
        assigned_add_on_sid: "your assigned add on sid",
        resource_sid: "your resource sid",
        sid: "your sid",
    };
    let response = client
        .fetch_incoming_phone_number_assigned_add_on_extension(args)
        .await
        .unwrap();
    println!("{:#?}", response);
}
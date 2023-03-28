#![allow(unused_imports)]
use twilio::TwilioClient;
use twilio::model::*;
#[tokio::main]
async fn main() {
    let client = TwilioClient::from_env();
    let account_sid = "your account sid";
    let country_code = "your country code";
    let response = client
        .list_available_phone_number_toll_free(account_sid, country_code)
        .area_code(1)
        .beta(true)
        .contains("your contains")
        .distance(1)
        .exclude_all_address_required(true)
        .exclude_foreign_address_required(true)
        .exclude_local_address_required(true)
        .fax_enabled(true)
        .in_lata("your in lata")
        .in_locality("your in locality")
        .in_postal_code("your in postal code")
        .in_rate_center("your in rate center")
        .in_region("your in region")
        .mms_enabled(true)
        .near_lat_long("your near lat long")
        .near_number("your near number")
        .page(1)
        .page_size(1)
        .page_token("your page token")
        .sms_enabled(true)
        .voice_enabled(true)
        .await
        .unwrap();
    println!("{:#?}", response);
}
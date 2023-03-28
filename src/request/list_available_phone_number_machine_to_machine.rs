use serde_json::json;
use crate::model::*;
use crate::TwilioClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct ListAvailablePhoneNumberMachineToMachineRequest<'a> {
    pub(crate) http_client: &'a TwilioClient,
    pub account_sid: String,
    pub area_code: Option<i64>,
    pub beta: Option<bool>,
    pub contains: Option<String>,
    pub country_code: String,
    pub distance: Option<i64>,
    pub exclude_all_address_required: Option<bool>,
    pub exclude_foreign_address_required: Option<bool>,
    pub exclude_local_address_required: Option<bool>,
    pub fax_enabled: Option<bool>,
    pub in_lata: Option<String>,
    pub in_locality: Option<String>,
    pub in_postal_code: Option<String>,
    pub in_rate_center: Option<String>,
    pub in_region: Option<String>,
    pub mms_enabled: Option<bool>,
    pub near_lat_long: Option<String>,
    pub near_number: Option<String>,
    pub page: Option<i64>,
    pub page_size: Option<i64>,
    pub page_token: Option<String>,
    pub sms_enabled: Option<bool>,
    pub voice_enabled: Option<bool>,
}
impl<'a> ListAvailablePhoneNumberMachineToMachineRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<serde_json::Value> {
        let mut r = self
            .http_client
            .client
            .get(
                &format!(
                    "/2010-04-01/Accounts/{account_sid}/AvailablePhoneNumbers/{country_code}/MachineToMachine.json",
                    account_sid = self.account_sid, country_code = self.country_code
                ),
            );
        if let Some(ref unwrapped) = self.area_code {
            r = r.query("AreaCode", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.beta {
            r = r.query("Beta", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.contains {
            r = r.query("Contains", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.distance {
            r = r.query("Distance", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.exclude_all_address_required {
            r = r.query("ExcludeAllAddressRequired", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.exclude_foreign_address_required {
            r = r.query("ExcludeForeignAddressRequired", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.exclude_local_address_required {
            r = r.query("ExcludeLocalAddressRequired", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.fax_enabled {
            r = r.query("FaxEnabled", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.in_lata {
            r = r.query("InLata", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.in_locality {
            r = r.query("InLocality", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.in_postal_code {
            r = r.query("InPostalCode", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.in_rate_center {
            r = r.query("InRateCenter", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.in_region {
            r = r.query("InRegion", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.mms_enabled {
            r = r.query("MmsEnabled", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.near_lat_long {
            r = r.query("NearLatLong", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.near_number {
            r = r.query("NearNumber", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.page {
            r = r.query("Page", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.page_size {
            r = r.query("PageSize", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.page_token {
            r = r.query("PageToken", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.sms_enabled {
            r = r.query("SmsEnabled", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.voice_enabled {
            r = r.query("VoiceEnabled", &unwrapped.to_string());
        }
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        res.json()
    }
    pub fn area_code(mut self, area_code: i64) -> Self {
        self.area_code = Some(area_code);
        self
    }
    pub fn beta(mut self, beta: bool) -> Self {
        self.beta = Some(beta);
        self
    }
    pub fn contains(mut self, contains: &str) -> Self {
        self.contains = Some(contains.to_owned());
        self
    }
    pub fn distance(mut self, distance: i64) -> Self {
        self.distance = Some(distance);
        self
    }
    pub fn exclude_all_address_required(
        mut self,
        exclude_all_address_required: bool,
    ) -> Self {
        self.exclude_all_address_required = Some(exclude_all_address_required);
        self
    }
    pub fn exclude_foreign_address_required(
        mut self,
        exclude_foreign_address_required: bool,
    ) -> Self {
        self.exclude_foreign_address_required = Some(exclude_foreign_address_required);
        self
    }
    pub fn exclude_local_address_required(
        mut self,
        exclude_local_address_required: bool,
    ) -> Self {
        self.exclude_local_address_required = Some(exclude_local_address_required);
        self
    }
    pub fn fax_enabled(mut self, fax_enabled: bool) -> Self {
        self.fax_enabled = Some(fax_enabled);
        self
    }
    pub fn in_lata(mut self, in_lata: &str) -> Self {
        self.in_lata = Some(in_lata.to_owned());
        self
    }
    pub fn in_locality(mut self, in_locality: &str) -> Self {
        self.in_locality = Some(in_locality.to_owned());
        self
    }
    pub fn in_postal_code(mut self, in_postal_code: &str) -> Self {
        self.in_postal_code = Some(in_postal_code.to_owned());
        self
    }
    pub fn in_rate_center(mut self, in_rate_center: &str) -> Self {
        self.in_rate_center = Some(in_rate_center.to_owned());
        self
    }
    pub fn in_region(mut self, in_region: &str) -> Self {
        self.in_region = Some(in_region.to_owned());
        self
    }
    pub fn mms_enabled(mut self, mms_enabled: bool) -> Self {
        self.mms_enabled = Some(mms_enabled);
        self
    }
    pub fn near_lat_long(mut self, near_lat_long: &str) -> Self {
        self.near_lat_long = Some(near_lat_long.to_owned());
        self
    }
    pub fn near_number(mut self, near_number: &str) -> Self {
        self.near_number = Some(near_number.to_owned());
        self
    }
    pub fn page(mut self, page: i64) -> Self {
        self.page = Some(page);
        self
    }
    pub fn page_size(mut self, page_size: i64) -> Self {
        self.page_size = Some(page_size);
        self
    }
    pub fn page_token(mut self, page_token: &str) -> Self {
        self.page_token = Some(page_token.to_owned());
        self
    }
    pub fn sms_enabled(mut self, sms_enabled: bool) -> Self {
        self.sms_enabled = Some(sms_enabled);
        self
    }
    pub fn voice_enabled(mut self, voice_enabled: bool) -> Self {
        self.voice_enabled = Some(voice_enabled);
        self
    }
}
impl<'a> ::std::future::IntoFuture
for ListAvailablePhoneNumberMachineToMachineRequest<'a> {
    type Output = httpclient::InMemoryResult<serde_json::Value>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
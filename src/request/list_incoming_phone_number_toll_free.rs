use serde_json::json;
use crate::model::*;
use crate::TwilioClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct ListIncomingPhoneNumberTollFreeRequest<'a> {
    pub(crate) http_client: &'a TwilioClient,
    pub account_sid: String,
    pub beta: Option<bool>,
    pub friendly_name: Option<String>,
    pub origin: Option<String>,
    pub page: Option<i64>,
    pub page_size: Option<i64>,
    pub page_token: Option<String>,
    pub phone_number: Option<String>,
}
impl<'a> ListIncomingPhoneNumberTollFreeRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<serde_json::Value> {
        let mut r = self
            .http_client
            .client
            .get(
                &format!(
                    "/2010-04-01/Accounts/{account_sid}/IncomingPhoneNumbers/TollFree.json",
                    account_sid = self.account_sid
                ),
            );
        if let Some(ref unwrapped) = self.beta {
            r = r.query("Beta", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.friendly_name {
            r = r.query("FriendlyName", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.origin {
            r = r.query("Origin", &unwrapped.to_string());
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
        if let Some(ref unwrapped) = self.phone_number {
            r = r.query("PhoneNumber", &unwrapped.to_string());
        }
        let res = r.send_awaiting_body().await?;
        res.json()
    }
    pub fn beta(mut self, beta: bool) -> Self {
        self.beta = Some(beta);
        self
    }
    pub fn friendly_name(mut self, friendly_name: &str) -> Self {
        self.friendly_name = Some(friendly_name.to_owned());
        self
    }
    pub fn origin(mut self, origin: &str) -> Self {
        self.origin = Some(origin.to_owned());
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
    pub fn phone_number(mut self, phone_number: &str) -> Self {
        self.phone_number = Some(phone_number.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture for ListIncomingPhoneNumberTollFreeRequest<'a> {
    type Output = httpclient::InMemoryResult<serde_json::Value>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
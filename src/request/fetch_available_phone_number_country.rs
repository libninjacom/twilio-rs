use serde_json::json;
use crate::model::*;
use crate::TwilioClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct FetchAvailablePhoneNumberCountryRequest<'a> {
    pub(crate) http_client: &'a TwilioClient,
    pub account_sid: String,
    pub country_code: String,
}
impl<'a> FetchAvailablePhoneNumberCountryRequest<'a> {
    pub async fn send(
        self,
    ) -> ::httpclient::InMemoryResult<ApiV2010AccountAvailablePhoneNumberCountry> {
        let mut r = self
            .http_client
            .client
            .get(
                &format!(
                    "/2010-04-01/Accounts/{account_sid}/AvailablePhoneNumbers/{country_code}.json",
                    account_sid = self.account_sid, country_code = self.country_code
                ),
            );
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        res.json()
    }
}
impl<'a> ::std::future::IntoFuture for FetchAvailablePhoneNumberCountryRequest<'a> {
    type Output = httpclient::InMemoryResult<ApiV2010AccountAvailablePhoneNumberCountry>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
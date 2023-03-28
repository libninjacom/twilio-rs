use serde_json::json;
use crate::model::*;
use crate::TwilioClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct CreateIncomingPhoneNumberMobileRequest<'a> {
    pub(crate) http_client: &'a TwilioClient,
    pub account_sid: String,
}
impl<'a> CreateIncomingPhoneNumberMobileRequest<'a> {
    pub async fn send(
        self,
    ) -> ::httpclient::InMemoryResult<
        ApiV2010AccountIncomingPhoneNumberIncomingPhoneNumberMobile,
    > {
        let mut r = self
            .http_client
            .client
            .post(
                &format!(
                    "/2010-04-01/Accounts/{account_sid}/IncomingPhoneNumbers/Mobile.json",
                    account_sid = self.account_sid
                ),
            );
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        res.json()
    }
}
impl<'a> ::std::future::IntoFuture for CreateIncomingPhoneNumberMobileRequest<'a> {
    type Output = httpclient::InMemoryResult<
        ApiV2010AccountIncomingPhoneNumberIncomingPhoneNumberMobile,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
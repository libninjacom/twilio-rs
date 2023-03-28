use serde_json::json;
use crate::model::*;
use crate::TwilioClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct CreateIncomingPhoneNumberTollFreeRequest<'a> {
    pub(crate) http_client: &'a TwilioClient,
    pub account_sid: String,
}
impl<'a> CreateIncomingPhoneNumberTollFreeRequest<'a> {
    pub async fn send(
        self,
    ) -> ::httpclient::InMemoryResult<
        ApiV2010AccountIncomingPhoneNumberIncomingPhoneNumberTollFree,
    > {
        let mut r = self
            .http_client
            .client
            .post(
                &format!(
                    "/2010-04-01/Accounts/{account_sid}/IncomingPhoneNumbers/TollFree.json",
                    account_sid = self.account_sid
                ),
            );
        let res = r.send_awaiting_body().await?;
        res.json()
    }
}
impl<'a> ::std::future::IntoFuture for CreateIncomingPhoneNumberTollFreeRequest<'a> {
    type Output = httpclient::InMemoryResult<
        ApiV2010AccountIncomingPhoneNumberIncomingPhoneNumberTollFree,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
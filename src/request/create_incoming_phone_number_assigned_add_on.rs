use serde_json::json;
use crate::model::*;
use crate::TwilioClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct CreateIncomingPhoneNumberAssignedAddOnRequest<'a> {
    pub(crate) http_client: &'a TwilioClient,
    pub account_sid: String,
    pub resource_sid: String,
}
impl<'a> CreateIncomingPhoneNumberAssignedAddOnRequest<'a> {
    pub async fn send(
        self,
    ) -> ::httpclient::InMemoryResult<
        ApiV2010AccountIncomingPhoneNumberIncomingPhoneNumberAssignedAddOn,
    > {
        let mut r = self
            .http_client
            .client
            .post(
                &format!(
                    "/2010-04-01/Accounts/{account_sid}/IncomingPhoneNumbers/{resource_sid}/AssignedAddOns.json",
                    account_sid = self.account_sid, resource_sid = self.resource_sid
                ),
            );
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        res.json()
    }
}
impl<'a> ::std::future::IntoFuture
for CreateIncomingPhoneNumberAssignedAddOnRequest<'a> {
    type Output = httpclient::InMemoryResult<
        ApiV2010AccountIncomingPhoneNumberIncomingPhoneNumberAssignedAddOn,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
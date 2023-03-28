use serde_json::json;
use crate::model::*;
use crate::TwilioClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct FetchIncomingPhoneNumberAssignedAddOnExtensionRequest<'a> {
    pub(crate) http_client: &'a TwilioClient,
    pub account_sid: String,
    pub assigned_add_on_sid: String,
    pub resource_sid: String,
    pub sid: String,
}
impl<'a> FetchIncomingPhoneNumberAssignedAddOnExtensionRequest<'a> {
    pub async fn send(
        self,
    ) -> ::httpclient::InMemoryResult<
        ApiV2010AccountIncomingPhoneNumberIncomingPhoneNumberAssignedAddOnIncomingPhoneNumberAssignedAddOnExtension,
    > {
        let mut r = self
            .http_client
            .client
            .get(
                &format!(
                    "/2010-04-01/Accounts/{account_sid}/IncomingPhoneNumbers/{resource_sid}/AssignedAddOns/{assigned_add_on_sid}/Extensions/{sid}.json",
                    account_sid = self.account_sid, assigned_add_on_sid = self
                    .assigned_add_on_sid, resource_sid = self.resource_sid, sid = self
                    .sid
                ),
            );
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        res.json()
    }
}
pub struct FetchIncomingPhoneNumberAssignedAddOnExtensionRequired<'a> {
    pub account_sid: &'a str,
    pub assigned_add_on_sid: &'a str,
    pub resource_sid: &'a str,
    pub sid: &'a str,
}
impl<'a> FetchIncomingPhoneNumberAssignedAddOnExtensionRequired<'a> {}
impl<'a> ::std::future::IntoFuture
for FetchIncomingPhoneNumberAssignedAddOnExtensionRequest<'a> {
    type Output = httpclient::InMemoryResult<
        ApiV2010AccountIncomingPhoneNumberIncomingPhoneNumberAssignedAddOnIncomingPhoneNumberAssignedAddOnExtension,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
use serde_json::json;
use crate::model::*;
use crate::TwilioClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct DeleteSipAuthCallsCredentialListMappingRequest<'a> {
    pub(crate) http_client: &'a TwilioClient,
    pub account_sid: String,
    pub domain_sid: String,
    pub sid: String,
}
impl<'a> DeleteSipAuthCallsCredentialListMappingRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<()> {
        let mut r = self
            .http_client
            .client
            .delete(
                &format!(
                    "/2010-04-01/Accounts/{account_sid}/SIP/Domains/{domain_sid}/Auth/Calls/CredentialListMappings/{sid}.json",
                    account_sid = self.account_sid, domain_sid = self.domain_sid, sid =
                    self.sid
                ),
            );
        let res = r.send_awaiting_body().await?;
        res.json()
    }
}
impl<'a> ::std::future::IntoFuture
for DeleteSipAuthCallsCredentialListMappingRequest<'a> {
    type Output = httpclient::InMemoryResult<()>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
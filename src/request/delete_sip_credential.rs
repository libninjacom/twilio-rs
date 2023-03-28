use serde_json::json;
use crate::model::*;
use crate::TwilioClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct DeleteSipCredentialRequest<'a> {
    pub(crate) http_client: &'a TwilioClient,
    pub account_sid: String,
    pub credential_list_sid: String,
    pub sid: String,
}
impl<'a> DeleteSipCredentialRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<()> {
        let mut r = self
            .http_client
            .client
            .delete(
                &format!(
                    "/2010-04-01/Accounts/{account_sid}/SIP/CredentialLists/{credential_list_sid}/Credentials/{sid}.json",
                    account_sid = self.account_sid, credential_list_sid = self
                    .credential_list_sid, sid = self.sid
                ),
            );
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        res.json()
    }
}
impl<'a> ::std::future::IntoFuture for DeleteSipCredentialRequest<'a> {
    type Output = httpclient::InMemoryResult<()>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
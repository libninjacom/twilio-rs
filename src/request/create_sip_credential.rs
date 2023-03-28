use serde_json::json;
use crate::model::*;
use crate::TwilioClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct CreateSipCredentialRequest<'a> {
    pub(crate) http_client: &'a TwilioClient,
    pub account_sid: String,
    pub credential_list_sid: String,
}
impl<'a> CreateSipCredentialRequest<'a> {
    pub async fn send(
        self,
    ) -> ::httpclient::InMemoryResult<ApiV2010AccountSipSipCredentialListSipCredential> {
        let mut r = self
            .http_client
            .client
            .post(
                &format!(
                    "/2010-04-01/Accounts/{account_sid}/SIP/CredentialLists/{credential_list_sid}/Credentials.json",
                    account_sid = self.account_sid, credential_list_sid = self
                    .credential_list_sid
                ),
            );
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        res.json()
    }
}
impl<'a> ::std::future::IntoFuture for CreateSipCredentialRequest<'a> {
    type Output = httpclient::InMemoryResult<
        ApiV2010AccountSipSipCredentialListSipCredential,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
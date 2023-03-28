use serde_json::json;
use crate::model::*;
use crate::TwilioClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct FetchAuthorizedConnectAppRequest<'a> {
    pub(crate) http_client: &'a TwilioClient,
    pub account_sid: String,
    pub connect_app_sid: String,
}
impl<'a> FetchAuthorizedConnectAppRequest<'a> {
    pub async fn send(
        self,
    ) -> ::httpclient::InMemoryResult<ApiV2010AccountAuthorizedConnectApp> {
        let mut r = self
            .http_client
            .client
            .get(
                &format!(
                    "/2010-04-01/Accounts/{account_sid}/AuthorizedConnectApps/{connect_app_sid}.json",
                    account_sid = self.account_sid, connect_app_sid = self
                    .connect_app_sid
                ),
            );
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        res.json()
    }
}
impl<'a> ::std::future::IntoFuture for FetchAuthorizedConnectAppRequest<'a> {
    type Output = httpclient::InMemoryResult<ApiV2010AccountAuthorizedConnectApp>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
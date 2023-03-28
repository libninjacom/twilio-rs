use serde_json::json;
use crate::model::*;
use crate::TwilioClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct CreateAccountRequest<'a> {
    pub(crate) http_client: &'a TwilioClient,
}
impl<'a> CreateAccountRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<ApiV2010Account> {
        let mut r = self.http_client.client.post("/2010-04-01/Accounts.json");
        let res = r.send_awaiting_body().await?;
        res.json()
    }
}
impl<'a> ::std::future::IntoFuture for CreateAccountRequest<'a> {
    type Output = httpclient::InMemoryResult<ApiV2010Account>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
use serde_json::json;
use crate::model::*;
use crate::TwilioClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct FetchRecordingAddOnResultRequest<'a> {
    pub(crate) http_client: &'a TwilioClient,
    pub account_sid: String,
    pub reference_sid: String,
    pub sid: String,
}
impl<'a> FetchRecordingAddOnResultRequest<'a> {
    pub async fn send(
        self,
    ) -> ::httpclient::InMemoryResult<ApiV2010AccountRecordingRecordingAddOnResult> {
        let mut r = self
            .http_client
            .client
            .get(
                &format!(
                    "/2010-04-01/Accounts/{account_sid}/Recordings/{reference_sid}/AddOnResults/{sid}.json",
                    account_sid = self.account_sid, reference_sid = self.reference_sid,
                    sid = self.sid
                ),
            );
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        res.json()
    }
}
impl<'a> ::std::future::IntoFuture for FetchRecordingAddOnResultRequest<'a> {
    type Output = httpclient::InMemoryResult<
        ApiV2010AccountRecordingRecordingAddOnResult,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
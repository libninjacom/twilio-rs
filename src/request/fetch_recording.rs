use serde_json::json;
use crate::model::*;
use crate::TwilioClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct FetchRecordingRequest<'a> {
    pub(crate) http_client: &'a TwilioClient,
    pub account_sid: String,
    pub include_soft_deleted: Option<bool>,
    pub sid: String,
}
impl<'a> FetchRecordingRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<ApiV2010AccountRecording> {
        let mut r = self
            .http_client
            .client
            .get(
                &format!(
                    "/2010-04-01/Accounts/{account_sid}/Recordings/{sid}.json",
                    account_sid = self.account_sid, sid = self.sid
                ),
            );
        if let Some(ref unwrapped) = self.include_soft_deleted {
            r = r.query("IncludeSoftDeleted", &unwrapped.to_string());
        }
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        res.json()
    }
    pub fn include_soft_deleted(mut self, include_soft_deleted: bool) -> Self {
        self.include_soft_deleted = Some(include_soft_deleted);
        self
    }
}
impl<'a> ::std::future::IntoFuture for FetchRecordingRequest<'a> {
    type Output = httpclient::InMemoryResult<ApiV2010AccountRecording>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
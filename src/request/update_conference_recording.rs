use serde_json::json;
use crate::model::*;
use crate::TwilioClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct UpdateConferenceRecordingRequest<'a> {
    pub(crate) http_client: &'a TwilioClient,
    pub account_sid: String,
    pub conference_sid: String,
    pub sid: String,
}
impl<'a> UpdateConferenceRecordingRequest<'a> {
    pub async fn send(
        self,
    ) -> ::httpclient::InMemoryResult<ApiV2010AccountConferenceConferenceRecording> {
        let mut r = self
            .http_client
            .client
            .post(
                &format!(
                    "/2010-04-01/Accounts/{account_sid}/Conferences/{conference_sid}/Recordings/{sid}.json",
                    account_sid = self.account_sid, conference_sid = self.conference_sid,
                    sid = self.sid
                ),
            );
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        res.json()
    }
}
impl<'a> ::std::future::IntoFuture for UpdateConferenceRecordingRequest<'a> {
    type Output = httpclient::InMemoryResult<
        ApiV2010AccountConferenceConferenceRecording,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
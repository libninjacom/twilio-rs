use serde_json::json;
use crate::model::*;
use crate::TwilioClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct DeleteRecordingAddOnResultPayloadRequest<'a> {
    pub(crate) http_client: &'a TwilioClient,
    pub account_sid: String,
    pub add_on_result_sid: String,
    pub reference_sid: String,
    pub sid: String,
}
impl<'a> DeleteRecordingAddOnResultPayloadRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<()> {
        let mut r = self
            .http_client
            .client
            .delete(
                &format!(
                    "/2010-04-01/Accounts/{account_sid}/Recordings/{reference_sid}/AddOnResults/{add_on_result_sid}/Payloads/{sid}.json",
                    account_sid = self.account_sid, add_on_result_sid = self
                    .add_on_result_sid, reference_sid = self.reference_sid, sid = self
                    .sid
                ),
            );
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        res.json()
    }
}
pub struct DeleteRecordingAddOnResultPayloadRequired<'a> {
    pub account_sid: &'a str,
    pub add_on_result_sid: &'a str,
    pub reference_sid: &'a str,
    pub sid: &'a str,
}
impl<'a> DeleteRecordingAddOnResultPayloadRequired<'a> {}
impl<'a> ::std::future::IntoFuture for DeleteRecordingAddOnResultPayloadRequest<'a> {
    type Output = httpclient::InMemoryResult<()>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
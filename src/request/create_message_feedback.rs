use serde_json::json;
use crate::model::*;
use crate::TwilioClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct CreateMessageFeedbackRequest<'a> {
    pub(crate) http_client: &'a TwilioClient,
    pub account_sid: String,
    pub message_sid: String,
}
impl<'a> CreateMessageFeedbackRequest<'a> {
    pub async fn send(
        self,
    ) -> ::httpclient::InMemoryResult<ApiV2010AccountMessageMessageFeedback> {
        let mut r = self
            .http_client
            .client
            .post(
                &format!(
                    "/2010-04-01/Accounts/{account_sid}/Messages/{message_sid}/Feedback.json",
                    account_sid = self.account_sid, message_sid = self.message_sid
                ),
            );
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        res.json()
    }
}
impl<'a> ::std::future::IntoFuture for CreateMessageFeedbackRequest<'a> {
    type Output = httpclient::InMemoryResult<ApiV2010AccountMessageMessageFeedback>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
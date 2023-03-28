use serde_json::json;
use crate::model::*;
use crate::TwilioClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct ListRecordingRequest<'a> {
    pub(crate) http_client: &'a TwilioClient,
    pub account_sid: String,
    pub call_sid: Option<String>,
    pub conference_sid: Option<String>,
    pub date_created: Option<chrono::DateTime<chrono::Utc>>,
    pub date_created_gt: Option<chrono::DateTime<chrono::Utc>>,
    pub date_created_lt: Option<chrono::DateTime<chrono::Utc>>,
    pub include_soft_deleted: Option<bool>,
    pub page: Option<i64>,
    pub page_size: Option<i64>,
    pub page_token: Option<String>,
}
impl<'a> ListRecordingRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<serde_json::Value> {
        let mut r = self
            .http_client
            .client
            .get(
                &format!(
                    "/2010-04-01/Accounts/{account_sid}/Recordings.json", account_sid =
                    self.account_sid
                ),
            );
        if let Some(ref unwrapped) = self.call_sid {
            r = r.query("CallSid", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.conference_sid {
            r = r.query("ConferenceSid", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.date_created {
            r = r.query("DateCreated", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.date_created_gt {
            r = r.query("DateCreated_gt", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.date_created_lt {
            r = r.query("DateCreated_lt", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.include_soft_deleted {
            r = r.query("IncludeSoftDeleted", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.page {
            r = r.query("Page", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.page_size {
            r = r.query("PageSize", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.page_token {
            r = r.query("PageToken", &unwrapped.to_string());
        }
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        res.json()
    }
    pub fn call_sid(mut self, call_sid: &str) -> Self {
        self.call_sid = Some(call_sid.to_owned());
        self
    }
    pub fn conference_sid(mut self, conference_sid: &str) -> Self {
        self.conference_sid = Some(conference_sid.to_owned());
        self
    }
    pub fn date_created(mut self, date_created: chrono::DateTime<chrono::Utc>) -> Self {
        self.date_created = Some(date_created);
        self
    }
    pub fn date_created_gt(
        mut self,
        date_created_gt: chrono::DateTime<chrono::Utc>,
    ) -> Self {
        self.date_created_gt = Some(date_created_gt);
        self
    }
    pub fn date_created_lt(
        mut self,
        date_created_lt: chrono::DateTime<chrono::Utc>,
    ) -> Self {
        self.date_created_lt = Some(date_created_lt);
        self
    }
    pub fn include_soft_deleted(mut self, include_soft_deleted: bool) -> Self {
        self.include_soft_deleted = Some(include_soft_deleted);
        self
    }
    pub fn page(mut self, page: i64) -> Self {
        self.page = Some(page);
        self
    }
    pub fn page_size(mut self, page_size: i64) -> Self {
        self.page_size = Some(page_size);
        self
    }
    pub fn page_token(mut self, page_token: &str) -> Self {
        self.page_token = Some(page_token.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture for ListRecordingRequest<'a> {
    type Output = httpclient::InMemoryResult<serde_json::Value>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
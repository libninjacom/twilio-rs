use serde_json::json;
use crate::model::*;
use crate::TwilioClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct ListCallRequest<'a> {
    pub(crate) http_client: &'a TwilioClient,
    pub account_sid: String,
    pub end_time: Option<chrono::DateTime<chrono::Utc>>,
    pub end_time_gt: Option<chrono::DateTime<chrono::Utc>>,
    pub end_time_lt: Option<chrono::DateTime<chrono::Utc>>,
    pub from: Option<String>,
    pub page: Option<i64>,
    pub page_size: Option<i64>,
    pub page_token: Option<String>,
    pub parent_call_sid: Option<String>,
    pub start_time: Option<chrono::DateTime<chrono::Utc>>,
    pub start_time_gt: Option<chrono::DateTime<chrono::Utc>>,
    pub start_time_lt: Option<chrono::DateTime<chrono::Utc>>,
    pub status: Option<String>,
    pub to: Option<String>,
}
impl<'a> ListCallRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<serde_json::Value> {
        let mut r = self
            .http_client
            .client
            .get(
                &format!(
                    "/2010-04-01/Accounts/{account_sid}/Calls.json", account_sid = self
                    .account_sid
                ),
            );
        if let Some(ref unwrapped) = self.end_time {
            r = r.query("EndTime", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.end_time_gt {
            r = r.query("EndTime_gt", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.end_time_lt {
            r = r.query("EndTime_lt", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.from {
            r = r.query("From", &unwrapped.to_string());
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
        if let Some(ref unwrapped) = self.parent_call_sid {
            r = r.query("ParentCallSid", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.start_time {
            r = r.query("StartTime", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.start_time_gt {
            r = r.query("StartTime_gt", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.start_time_lt {
            r = r.query("StartTime_lt", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.status {
            r = r.query("Status", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.to {
            r = r.query("To", &unwrapped.to_string());
        }
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        res.json()
    }
    pub fn end_time(mut self, end_time: chrono::DateTime<chrono::Utc>) -> Self {
        self.end_time = Some(end_time);
        self
    }
    pub fn end_time_gt(mut self, end_time_gt: chrono::DateTime<chrono::Utc>) -> Self {
        self.end_time_gt = Some(end_time_gt);
        self
    }
    pub fn end_time_lt(mut self, end_time_lt: chrono::DateTime<chrono::Utc>) -> Self {
        self.end_time_lt = Some(end_time_lt);
        self
    }
    pub fn from(mut self, from: &str) -> Self {
        self.from = Some(from.to_owned());
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
    pub fn parent_call_sid(mut self, parent_call_sid: &str) -> Self {
        self.parent_call_sid = Some(parent_call_sid.to_owned());
        self
    }
    pub fn start_time(mut self, start_time: chrono::DateTime<chrono::Utc>) -> Self {
        self.start_time = Some(start_time);
        self
    }
    pub fn start_time_gt(
        mut self,
        start_time_gt: chrono::DateTime<chrono::Utc>,
    ) -> Self {
        self.start_time_gt = Some(start_time_gt);
        self
    }
    pub fn start_time_lt(
        mut self,
        start_time_lt: chrono::DateTime<chrono::Utc>,
    ) -> Self {
        self.start_time_lt = Some(start_time_lt);
        self
    }
    pub fn status(mut self, status: &str) -> Self {
        self.status = Some(status.to_owned());
        self
    }
    pub fn to(mut self, to: &str) -> Self {
        self.to = Some(to.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture for ListCallRequest<'a> {
    type Output = httpclient::InMemoryResult<serde_json::Value>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
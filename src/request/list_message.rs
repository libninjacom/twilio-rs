use serde_json::json;
use crate::model::*;
use crate::TwilioClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct ListMessageRequest<'a> {
    pub(crate) http_client: &'a TwilioClient,
    pub account_sid: String,
    pub date_sent: Option<chrono::DateTime<chrono::Utc>>,
    pub date_sent_gt: Option<chrono::DateTime<chrono::Utc>>,
    pub date_sent_lt: Option<chrono::DateTime<chrono::Utc>>,
    pub from: Option<String>,
    pub page: Option<i64>,
    pub page_size: Option<i64>,
    pub page_token: Option<String>,
    pub to: Option<String>,
}
impl<'a> ListMessageRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<serde_json::Value> {
        let mut r = self
            .http_client
            .client
            .get(
                &format!(
                    "/2010-04-01/Accounts/{account_sid}/Messages.json", account_sid =
                    self.account_sid
                ),
            );
        if let Some(ref unwrapped) = self.date_sent {
            r = r.query("DateSent", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.date_sent_gt {
            r = r.query("DateSent_gt", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.date_sent_lt {
            r = r.query("DateSent_lt", &unwrapped.to_string());
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
        if let Some(ref unwrapped) = self.to {
            r = r.query("To", &unwrapped.to_string());
        }
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        res.json()
    }
    pub fn date_sent(mut self, date_sent: chrono::DateTime<chrono::Utc>) -> Self {
        self.date_sent = Some(date_sent);
        self
    }
    pub fn date_sent_gt(mut self, date_sent_gt: chrono::DateTime<chrono::Utc>) -> Self {
        self.date_sent_gt = Some(date_sent_gt);
        self
    }
    pub fn date_sent_lt(mut self, date_sent_lt: chrono::DateTime<chrono::Utc>) -> Self {
        self.date_sent_lt = Some(date_sent_lt);
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
    pub fn to(mut self, to: &str) -> Self {
        self.to = Some(to.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture for ListMessageRequest<'a> {
    type Output = httpclient::InMemoryResult<serde_json::Value>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
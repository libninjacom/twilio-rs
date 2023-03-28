use serde_json::json;
use crate::model::*;
use crate::TwilioClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct ListUsageTriggerRequest<'a> {
    pub(crate) http_client: &'a TwilioClient,
    pub account_sid: String,
    pub page: Option<i64>,
    pub page_size: Option<i64>,
    pub page_token: Option<String>,
    pub recurring: Option<String>,
    pub trigger_by: Option<String>,
    pub usage_category: Option<String>,
}
impl<'a> ListUsageTriggerRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<serde_json::Value> {
        let mut r = self
            .http_client
            .client
            .get(
                &format!(
                    "/2010-04-01/Accounts/{account_sid}/Usage/Triggers.json", account_sid
                    = self.account_sid
                ),
            );
        if let Some(ref unwrapped) = self.page {
            r = r.query("Page", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.page_size {
            r = r.query("PageSize", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.page_token {
            r = r.query("PageToken", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.recurring {
            r = r.query("Recurring", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.trigger_by {
            r = r.query("TriggerBy", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.usage_category {
            r = r.query("UsageCategory", &unwrapped.to_string());
        }
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        res.json()
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
    pub fn recurring(mut self, recurring: &str) -> Self {
        self.recurring = Some(recurring.to_owned());
        self
    }
    pub fn trigger_by(mut self, trigger_by: &str) -> Self {
        self.trigger_by = Some(trigger_by.to_owned());
        self
    }
    pub fn usage_category(mut self, usage_category: &str) -> Self {
        self.usage_category = Some(usage_category.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture for ListUsageTriggerRequest<'a> {
    type Output = httpclient::InMemoryResult<serde_json::Value>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
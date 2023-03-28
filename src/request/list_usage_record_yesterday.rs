use serde_json::json;
use crate::model::*;
use crate::TwilioClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct ListUsageRecordYesterdayRequest<'a> {
    pub(crate) http_client: &'a TwilioClient,
    pub account_sid: String,
    pub category: Option<String>,
    pub end_date: Option<chrono::NaiveDate>,
    pub include_subaccounts: Option<bool>,
    pub page: Option<i64>,
    pub page_size: Option<i64>,
    pub page_token: Option<String>,
    pub start_date: Option<chrono::NaiveDate>,
}
impl<'a> ListUsageRecordYesterdayRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<serde_json::Value> {
        let mut r = self
            .http_client
            .client
            .get(
                &format!(
                    "/2010-04-01/Accounts/{account_sid}/Usage/Records/Yesterday.json",
                    account_sid = self.account_sid
                ),
            );
        if let Some(ref unwrapped) = self.category {
            r = r.query("Category", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.end_date {
            r = r.query("EndDate", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.include_subaccounts {
            r = r.query("IncludeSubaccounts", &unwrapped.to_string());
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
        if let Some(ref unwrapped) = self.start_date {
            r = r.query("StartDate", &unwrapped.to_string());
        }
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        res.json()
    }
    pub fn category(mut self, category: &str) -> Self {
        self.category = Some(category.to_owned());
        self
    }
    pub fn end_date(mut self, end_date: chrono::NaiveDate) -> Self {
        self.end_date = Some(end_date);
        self
    }
    pub fn include_subaccounts(mut self, include_subaccounts: bool) -> Self {
        self.include_subaccounts = Some(include_subaccounts);
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
    pub fn start_date(mut self, start_date: chrono::NaiveDate) -> Self {
        self.start_date = Some(start_date);
        self
    }
}
impl<'a> ::std::future::IntoFuture for ListUsageRecordYesterdayRequest<'a> {
    type Output = httpclient::InMemoryResult<serde_json::Value>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
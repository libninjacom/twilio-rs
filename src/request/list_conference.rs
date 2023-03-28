use serde_json::json;
use crate::model::*;
use crate::TwilioClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct ListConferenceRequest<'a> {
    pub(crate) http_client: &'a TwilioClient,
    pub account_sid: String,
    pub date_created: Option<chrono::NaiveDate>,
    pub date_created_gt: Option<chrono::NaiveDate>,
    pub date_created_lt: Option<chrono::NaiveDate>,
    pub date_updated: Option<chrono::NaiveDate>,
    pub date_updated_gt: Option<chrono::NaiveDate>,
    pub date_updated_lt: Option<chrono::NaiveDate>,
    pub friendly_name: Option<String>,
    pub page: Option<i64>,
    pub page_size: Option<i64>,
    pub page_token: Option<String>,
    pub status: Option<String>,
}
impl<'a> ListConferenceRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<serde_json::Value> {
        let mut r = self
            .http_client
            .client
            .get(
                &format!(
                    "/2010-04-01/Accounts/{account_sid}/Conferences.json", account_sid =
                    self.account_sid
                ),
            );
        if let Some(ref unwrapped) = self.date_created {
            r = r.query("DateCreated", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.date_created_gt {
            r = r.query("DateCreated_gt", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.date_created_lt {
            r = r.query("DateCreated_lt", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.date_updated {
            r = r.query("DateUpdated", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.date_updated_gt {
            r = r.query("DateUpdated_gt", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.date_updated_lt {
            r = r.query("DateUpdated_lt", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.friendly_name {
            r = r.query("FriendlyName", &unwrapped.to_string());
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
        if let Some(ref unwrapped) = self.status {
            r = r.query("Status", &unwrapped.to_string());
        }
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        res.json()
    }
    pub fn date_created(mut self, date_created: chrono::NaiveDate) -> Self {
        self.date_created = Some(date_created);
        self
    }
    pub fn date_created_gt(mut self, date_created_gt: chrono::NaiveDate) -> Self {
        self.date_created_gt = Some(date_created_gt);
        self
    }
    pub fn date_created_lt(mut self, date_created_lt: chrono::NaiveDate) -> Self {
        self.date_created_lt = Some(date_created_lt);
        self
    }
    pub fn date_updated(mut self, date_updated: chrono::NaiveDate) -> Self {
        self.date_updated = Some(date_updated);
        self
    }
    pub fn date_updated_gt(mut self, date_updated_gt: chrono::NaiveDate) -> Self {
        self.date_updated_gt = Some(date_updated_gt);
        self
    }
    pub fn date_updated_lt(mut self, date_updated_lt: chrono::NaiveDate) -> Self {
        self.date_updated_lt = Some(date_updated_lt);
        self
    }
    pub fn friendly_name(mut self, friendly_name: &str) -> Self {
        self.friendly_name = Some(friendly_name.to_owned());
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
    pub fn status(mut self, status: &str) -> Self {
        self.status = Some(status.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture for ListConferenceRequest<'a> {
    type Output = httpclient::InMemoryResult<serde_json::Value>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}
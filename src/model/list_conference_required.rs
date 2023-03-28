
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListConferenceRequired {
    #[serde(rename = "AccountSid")]
    pub account_sid: String,
    #[serde(rename = "DateCreated_gt")]
    pub date_created_gt: chrono::NaiveDate,
    #[serde(rename = "DateCreated_lt")]
    pub date_created_lt: chrono::NaiveDate,
    #[serde(rename = "DateUpdated_gt")]
    pub date_updated_gt: chrono::NaiveDate,
    #[serde(rename = "DateUpdated_lt")]
    pub date_updated_lt: chrono::NaiveDate,
}
impl std::fmt::Display for ListConferenceRequired {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
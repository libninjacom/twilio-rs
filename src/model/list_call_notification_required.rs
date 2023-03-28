
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListCallNotificationRequired {
    #[serde(rename = "AccountSid")]
    pub account_sid: String,
    #[serde(rename = "CallSid")]
    pub call_sid: String,
    #[serde(rename = "MessageDate_gt")]
    pub message_date_gt: chrono::NaiveDate,
    #[serde(rename = "MessageDate_lt")]
    pub message_date_lt: chrono::NaiveDate,
}
impl std::fmt::Display for ListCallNotificationRequired {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
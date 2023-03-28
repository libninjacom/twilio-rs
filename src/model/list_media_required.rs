
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListMediaRequired {
    #[serde(rename = "AccountSid")]
    pub account_sid: String,
    #[serde(rename = "DateCreated_gt")]
    pub date_created_gt: chrono::DateTime<chrono::Utc>,
    #[serde(rename = "DateCreated_lt")]
    pub date_created_lt: chrono::DateTime<chrono::Utc>,
    #[serde(rename = "MessageSid")]
    pub message_sid: String,
}
impl std::fmt::Display for ListMediaRequired {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
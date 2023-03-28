
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListCallRequired {
    #[serde(rename = "AccountSid")]
    pub account_sid: String,
    #[serde(rename = "EndTime_gt")]
    pub end_time_gt: chrono::DateTime<chrono::Utc>,
    #[serde(rename = "EndTime_lt")]
    pub end_time_lt: chrono::DateTime<chrono::Utc>,
    #[serde(rename = "StartTime_gt")]
    pub start_time_gt: chrono::DateTime<chrono::Utc>,
    #[serde(rename = "StartTime_lt")]
    pub start_time_lt: chrono::DateTime<chrono::Utc>,
}
impl std::fmt::Display for ListCallRequired {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
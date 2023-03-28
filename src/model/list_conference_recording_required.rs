
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListConferenceRecordingRequired {
    #[serde(rename = "AccountSid")]
    pub account_sid: String,
    #[serde(rename = "ConferenceSid")]
    pub conference_sid: String,
    #[serde(rename = "DateCreated_gt")]
    pub date_created_gt: chrono::NaiveDate,
    #[serde(rename = "DateCreated_lt")]
    pub date_created_lt: chrono::NaiveDate,
}
impl std::fmt::Display for ListConferenceRecordingRequired {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
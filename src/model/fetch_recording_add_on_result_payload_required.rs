
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FetchRecordingAddOnResultPayloadRequired {
    #[serde(rename = "AccountSid")]
    pub account_sid: String,
    #[serde(rename = "AddOnResultSid")]
    pub add_on_result_sid: String,
    #[serde(rename = "ReferenceSid")]
    pub reference_sid: String,
    #[serde(rename = "Sid")]
    pub sid: String,
}
impl std::fmt::Display for FetchRecordingAddOnResultPayloadRequired {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
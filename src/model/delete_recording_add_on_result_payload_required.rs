
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DeleteRecordingAddOnResultPayloadRequired {
    #[serde(rename = "AccountSid")]
    pub account_sid: String,
    #[serde(rename = "AddOnResultSid")]
    pub add_on_result_sid: String,
    #[serde(rename = "ReferenceSid")]
    pub reference_sid: String,
    #[serde(rename = "Sid")]
    pub sid: String,
}
impl std::fmt::Display for DeleteRecordingAddOnResultPayloadRequired {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
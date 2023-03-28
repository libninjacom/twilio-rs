
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FetchIncomingPhoneNumberAssignedAddOnExtensionRequired {
    #[serde(rename = "AccountSid")]
    pub account_sid: String,
    #[serde(rename = "AssignedAddOnSid")]
    pub assigned_add_on_sid: String,
    #[serde(rename = "ResourceSid")]
    pub resource_sid: String,
    #[serde(rename = "Sid")]
    pub sid: String,
}
impl std::fmt::Display for FetchIncomingPhoneNumberAssignedAddOnExtensionRequired {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
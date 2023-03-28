
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ApiV2010AccountIncomingPhoneNumberIncomingPhoneNumberMobile {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_sid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_requirements: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_sid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub beta: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bundle_sid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capabilities: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_created: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_updated: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emergency_address_sid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emergency_address_status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emergency_status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_sid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sms_application_sid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sms_fallback_method: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sms_fallback_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sms_method: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sms_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_callback: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_callback_method: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trunk_sid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_application_sid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_caller_id_lookup: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_fallback_method: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_fallback_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_method: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_receive_mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_url: Option<String>,
}
impl std::fmt::Display for ApiV2010AccountIncomingPhoneNumberIncomingPhoneNumberMobile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
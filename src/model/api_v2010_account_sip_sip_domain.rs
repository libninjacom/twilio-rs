
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ApiV2010AccountSipSipDomain {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_sid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub byoc_trunk_sid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_created: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_updated: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emergency_caller_sid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emergency_calling_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secure: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sip_registration: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subresource_uris: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_fallback_method: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_fallback_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_method: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_status_callback_method: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_status_callback_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_url: Option<String>,
}
impl std::fmt::Display for ApiV2010AccountSipSipDomain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
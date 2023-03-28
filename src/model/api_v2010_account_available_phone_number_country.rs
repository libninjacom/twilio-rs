
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ApiV2010AccountAvailablePhoneNumberCountry {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub beta: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subresource_uris: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
}
impl std::fmt::Display for ApiV2010AccountAvailablePhoneNumberCountry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
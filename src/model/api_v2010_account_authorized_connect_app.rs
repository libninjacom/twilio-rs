
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ApiV2010AccountAuthorizedConnectApp {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_sid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connect_app_company_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connect_app_description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connect_app_friendly_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connect_app_homepage_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connect_app_sid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_created: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_updated: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
}
impl std::fmt::Display for ApiV2010AccountAuthorizedConnectApp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
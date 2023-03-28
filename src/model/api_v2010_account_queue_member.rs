
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ApiV2010AccountQueueMember {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub call_sid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_enqueued: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queue_sid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wait_time: Option<i64>,
}
impl std::fmt::Display for ApiV2010AccountQueueMember {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
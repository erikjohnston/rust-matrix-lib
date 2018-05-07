use serde_json;
use serde_json::Value;

use ser::NamedHash;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Event {
    pub sender: String,
    #[serde(rename = "type")]
    pub etype: String,
    pub state_key: Option<String>,
    pub room_id: String,
    pub event_id: String,
    pub prev_events: Vec<(String, NamedHash)>,
    pub redacts: Option<String>,

    pub content: serde_json::Map<String, Value>,
}

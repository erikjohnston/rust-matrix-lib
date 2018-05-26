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

    pub depth: u64,

    pub content: serde_json::Map<String, Value>,
}

pub trait EventBase {
    fn get_room_id(&self) -> &str;
    fn get_event_id(&self) -> &str;
    fn get_sender(&self) -> &str;
    fn get_type(&self) -> &str;
    fn get_state_key(&self) -> Option<&str>;

    fn get_redacts(&self) -> Option<&str>;
    fn get_single_prev_event_id(&self) -> Option<&str>;

    fn get_content(&self) -> &serde_json::Map<String, Value>;
}

impl EventBase for Event {
    fn get_room_id(&self) -> &str {
        &self.room_id
    }
    fn get_event_id(&self) -> &str {
        &self.event_id
    }
    fn get_sender(&self) -> &str {
        &self.sender
    }
    fn get_type(&self) -> &str {
        &self.etype
    }
    fn get_state_key(&self) -> Option<&str> {
        self.state_key.as_ref().map(|s| s as &str)
    }

    fn get_redacts(&self) -> Option<&str> {
        self.redacts.as_ref().map(|s| s as &str)
    }
    fn get_single_prev_event_id(&self) -> Option<&str> {
        if self.prev_events.len() == 1 {
            Some(&self.prev_events[0].0)
        } else {
            None
        }
    }

    fn get_content(&self) -> &serde_json::Map<String, Value> {
        &self.content
    }
}

impl<'a, E> EventBase for &'a E where E: EventBase {
    fn get_room_id(&self) -> &str {
        (*self).get_room_id()
    }
    fn get_event_id(&self) -> &str {
        (*self).get_event_id()
    }
    fn get_sender(&self) -> &str {
        (*self).get_sender()
    }
    fn get_type(&self) -> &str {
        (*self).get_type()
    }
    fn get_state_key(&self) -> Option<&str> {
        (*self).get_state_key()
    }

    fn get_redacts(&self) -> Option<&str> {
        (*self).get_redacts()
    }
    fn get_single_prev_event_id(&self) -> Option<&str> {
        (*self).get_single_prev_event_id()
    }

    fn get_content(&self) -> &serde_json::Map<String, Value> {
        (*self).get_content()
    }
}

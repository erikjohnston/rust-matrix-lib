use std::collections::HashMap;

use serde_json;
use serde_json::Value;


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Event {
    pub sender: String,
    #[serde(rename = "type")]
    pub etype: String,
    pub state_key: Option<String>,
    pub room_id: String,
    pub event_id: String,
    pub prev_events: Vec<(String, HashMap<String, Hash>)>,
    pub redacts: Option<String>,

    pub content: serde_json::Map<String, Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Hash(#[serde(with = "serialize_hash")] Vec<u8>);


mod serialize_hash {
    use base64;
    use serde;
    use serde::Deserialize;

    pub fn serialize<S>(signature: &Vec<u8>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&base64::encode_config(
            &signature[..],
            base64::STANDARD_NO_PAD,
        ))
    }

    pub fn deserialize<'de, D>(d: D) -> Result<Vec<u8>, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = try!(String::deserialize(d));

        let bytes = base64::decode_config(&s, base64::STANDARD_NO_PAD)
            .map_err(|e| serde::de::Error::custom(e))?;

        Ok(bytes.into())
    }
}

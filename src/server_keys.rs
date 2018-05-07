use std::collections::BTreeMap;

use chrono::{self, Timelike};
use sodiumoxide::crypto::sign::PublicKey;

use ser::NamedHash;

#[derive(Debug, Clone, Serialize, Deserialize, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct VerifyKey {
    #[serde(with = "::ser::serialize_base64")]
    pub key: PublicKey,
}

#[derive(Debug, Clone, Serialize, Deserialize, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct KeyApiResponse {
    server_name: String,
    tls_fingerprints: Vec<NamedHash>,
    valid_until_ts: u64,
    verify_keys: BTreeMap<String, VerifyKey>,
    old_verify_keys: BTreeMap<String, VerifyKey>,
}

impl KeyApiResponse {
    pub fn new(server_name: &str) -> KeyApiResponse {
        let now = chrono::Utc::now() + chrono::Duration::days(1);
        let valid_until_ts = now.timestamp() * 1000 + now.time().nanosecond() as i64 / 1000000;

        KeyApiResponse {
            server_name: server_name.to_string(),
            valid_until_ts: valid_until_ts as u64,
            verify_keys: BTreeMap::new(),
            tls_fingerprints: Vec::new(),
            old_verify_keys: BTreeMap::new(),
        }
    }
}

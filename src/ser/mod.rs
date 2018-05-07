use sodiumoxide::crypto::sign::{PublicKey, Signature};

use sha2::{Digest, Sha256};

#[derive(Debug, Clone, Serialize, Deserialize, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum NamedHash {
    #[serde(rename = "sha256")]
    Sha256(#[serde(with = "::ser::serialize_base64")] [u8; 32]),
}

impl NamedHash {
    pub fn create_sha256(s: &[u8]) -> NamedHash {
        let b = Sha256::digest(s);
        let mut a = [0; 32];
        a.copy_from_slice(&b);
        NamedHash::Sha256(a)
    }

    pub fn matches(&self, s: &[u8]) -> bool {
        match *self {
            NamedHash::Sha256(ref h) => &Sha256::digest(s)[..] == h,
        }
    }
}

pub trait SliceConvert: Sized {
    fn from_slice(s: &[u8]) -> Option<Self>;
    fn as_slice(&self) -> &[u8];
}

/// Helper module for (de)serializing objects as base64 slices.
pub mod serialize_base64 {
    use super::SliceConvert;
    use base64;
    use serde;
    use serde::Deserialize;

    pub fn serialize<T, S>(signature: &T, serializer: S) -> Result<S::Ok, S::Error>
    where
        T: SliceConvert,
        S: serde::Serializer,
    {
        serializer.serialize_str(&base64::encode_config(
            signature.as_slice(),
            base64::STANDARD_NO_PAD,
        ))
    }

    pub fn deserialize<'de, T, D>(d: D) -> Result<T, D::Error>
    where
        T: SliceConvert,
        D: serde::Deserializer<'de>,
    {
        let s = try!(String::deserialize(d));

        let bytes = base64::decode_config(&s, base64::STANDARD_NO_PAD)
            .map_err(|e| serde::de::Error::custom(e))?;

        T::from_slice(&bytes).ok_or_else(|| serde::de::Error::custom("invalid value"))
    }
}

impl SliceConvert for Signature {
    fn from_slice(s: &[u8]) -> Option<Self> {
        Signature::from_slice(s)
    }

    fn as_slice(&self) -> &[u8] {
        &self[..]
    }
}

impl SliceConvert for PublicKey {
    fn from_slice(s: &[u8]) -> Option<Self> {
        PublicKey::from_slice(s)
    }

    fn as_slice(&self) -> &[u8] {
        &self[..]
    }
}

impl SliceConvert for [u8; 32] {
    fn from_slice(s: &[u8]) -> Option<Self> {
        if s.len() == 32 {
            let mut a = [0; 32];
            a.copy_from_slice(s);
            Some(a)
        } else {
            None
        }
    }

    fn as_slice(&self) -> &[u8] {
        self
    }
}

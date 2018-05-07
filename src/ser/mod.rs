use std::ops::{Index, RangeFull};

use sodiumoxide::crypto::sign::{PublicKey, Signature};

pub trait SliceConvert: Index<RangeFull, Output = [u8]> + Sized {
    fn from_slice(s: &[u8]) -> Option<Self>;
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
            &signature[..],
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
}

impl SliceConvert for PublicKey {
    fn from_slice(s: &[u8]) -> Option<Self> {
        PublicKey::from_slice(s)
    }
}

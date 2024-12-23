pub mod base64 {
    use serde::{Serialize, Deserialize, de};
    use serde::{Deserializer, Serializer};
    use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};

    pub fn serialize<S: Serializer>(v: &Vec<u8>, s: S) -> Result<S::Ok, S::Error> {
        let base64 = &BASE64.encode(v);
        String::serialize(&base64, s)
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<Vec<u8>, D::Error> {
        let s = String::deserialize(d)?;
        BASE64.decode(s).map_err(de::Error::custom)
    }
}
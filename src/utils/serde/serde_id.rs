//! 通常 id 是 Option<u64> 类型，需要序列化成 Option<String> \
//! use #[serde(with = "serde_id")]
use serde::{self, Deserialize, Deserializer, Serializer};

/// 序列化 Option<u64> 为 Option<String>
pub fn serialize<S>(id: &Option<u64>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    if let Some(id) = id {
        serializer.serialize_some(&id.to_string())
    } else {
        serializer.serialize_none()
    }
}

/// 反序列化 Option<String> 为 Option<u64>
pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<u64>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = Option::<String>::deserialize(deserializer)?;
    let id = s.and_then(|s| s.parse::<u64>().ok());
    Ok(id)
}

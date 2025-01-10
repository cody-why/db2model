//! 通常 id 是 Option<i64> 类型，需要序列化成 Option<String> \
//! use #[serde(with = "serde_id")]
use serde::{self, Deserialize, Deserializer, Serializer};

/// 序列化 Option<i64> 为 Option<String>
pub fn serialize<S, T>(id: &Option<T>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    T: ToString,
{
    if let Some(id) = id {
        serializer.serialize_some(&id.to_string())
    } else {
        serializer.serialize_none()
    }
}

/// 反序列化 Option<String> 为 Option<i64>
pub fn deserialize<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: std::str::FromStr,
{
    let s = Option::<String>::deserialize(deserializer)?;
    let id = s.and_then(|s| s.parse::<T>().ok());
    Ok(id)
}

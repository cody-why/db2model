//! 通常 id 是 Vec<i64> 类型，需要序列化成 Vec<String> \
//! use #[serde(with = "serde_id_vec")]
use serde::{self, Deserialize, Deserializer, Serializer};

/// 序列化 Vec<i64> 为 Vec<String>
pub fn serialize<S, T>(id: &[T], serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    T: ToString,
{
    let v = id.iter().map(|id| id.to_string()).collect::<Vec<String>>();
    serializer.collect_seq(v)
}

/// 反序列化 Vec<String> 为 Vec<i64>
pub fn deserialize<'de, D, T>(deserializer: D) -> Result<Vec<T>, D::Error>
where
    D: Deserializer<'de>,
    T: std::str::FromStr,
{
    let s = Vec::<String>::deserialize(deserializer)?;
    let id = s.iter().flat_map(|s| s.parse::<T>().ok()).collect();
    Ok(id)
}

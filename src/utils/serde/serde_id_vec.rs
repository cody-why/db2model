//! 通常 id 是 Vec<u64> 类型，需要序列化成 Vec<String> \
//! use #[serde(with = "serde_id_vec")]
use serde::{self, Deserialize, Deserializer, Serializer};

/// 序列化 Vec<u64> 为 Vec<String>
pub fn serialize<S>(id: &[u64], serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let v = id.iter().map(|id| id.to_string()).collect::<Vec<String>>();
    serializer.collect_seq(v)
}

/// 反序列化 Vec<String> 为 Vec<u64>
pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<u64>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = Vec::<String>::deserialize(deserializer)?;
    let id = s.iter().flat_map(|s| s.parse::<u64>().ok()).collect();
    Ok(id)
}

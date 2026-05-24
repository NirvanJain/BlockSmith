use serde::{
    de::DeserializeOwned,
    Serialize,
};

pub fn serialize<T>(
    value: &T,
) -> Result<String, serde_json::Error>
where
    T: Serialize,
{
    serde_json::to_string(value)
}

pub fn deserialize<T>(
    value: &str,
) -> Result<T, serde_json::Error>
where
    T: DeserializeOwned,
{
    serde_json::from_str(value)
}
use serde::{
    de::DeserializeOwned,
    Serialize,
};

pub fn to_json<T>(
    value: &T,
) -> Result<String, serde_json::Error>
where
    T: Serialize,
{
    serde_json::to_string(value)
}

pub fn from_json<T>(
    json: &str,
) -> Result<T, serde_json::Error>
where
    T: DeserializeOwned,
{
    serde_json::from_str(json)
}
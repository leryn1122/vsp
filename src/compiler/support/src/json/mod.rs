use anyhow::anyhow;
use serde::de::DeserializeOwned;

pub fn from_json<T: DeserializeOwned>(
  str: &'static str,
  json: &serde_json::Value,
) -> anyhow::Result<T> {
  let res = serde_json::from_value(json.clone())
    .map_err(|e| format!("Failed to deserialize {str}: {e}; {json}"));
  res.map_err(|e| anyhow!(e))
}

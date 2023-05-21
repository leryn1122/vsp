use serde::de::DeserializeOwned;
use vsp_error::VspError;
use vsp_error::VspResult;

pub fn from_json<T: DeserializeOwned>(str: &'static str, json: &serde_json::Value) -> VspResult<T> {
  let res = serde_json::from_value(json.clone())
    .map_err(|e| VspError::new(format!("Failed to deserialize {str}: {e}; {json}")));
  res
}

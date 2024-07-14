use serde_json::Value as JsonValue;

#[derive(Debug)]
pub struct JsonError {}

impl std::fmt::Display for JsonError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Error parsing JSON")
    }
}

impl std::error::Error for JsonError {}


pub fn boolean_int<'de, D: serde::de::Deserializer<'de>>(deserializer: D) -> Result<bool, D::Error> {
    Ok(match serde::de::Deserialize::deserialize(deserializer)? {
        JsonValue::Bool(b) => b,
        JsonValue::Number(num) => {
            num.as_i64()
                .ok_or(serde::de::Error::custom("Invalid number"))?
                != 0
        }
        JsonValue::Null => false,
        _ => return Err(serde::de::Error::custom("Wrong type, expected boolean")),
    })
}

pub fn bool(json: &serde_json::Value) -> std::result::Result<bool, JsonError> {
    json.as_bool().ok_or(JsonError {

    })
}

pub fn object(
    json: &serde_json::Value,
) -> std::result::Result<&serde_json::Map<String, serde_json::Value>, JsonError> {
    json.as_object().ok_or(JsonError {})
}

pub fn float(json: &serde_json::Value) -> std::result::Result<f64, JsonError> {
    json.as_f64().ok_or(JsonError {})
}

pub fn is_valid(js: &String) -> bool {
    serde_json::from_str::<serde_json::Value>(&js).is_ok()
}
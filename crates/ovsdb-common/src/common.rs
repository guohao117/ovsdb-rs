use serde::de::{self, Deserializer};
use serde::{Deserialize, Serialize};
const SET_HEADER: &str = "set";

// ovsdb atomic types
// https://tools.ietf.org/html/rfc7047#section-3.1.1
#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AtomicType {
    Integer,
    String,
    Real,
    Boolean,
    Uuid,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum Set<T> {
    One(T),
    Many(Vec<T>),
}

pub fn serialize_set<T, S>(set: &Option<Set<T>>, serializer: S) -> Result<S::Ok, S::Error>
where
    T: serde::Serialize,
    S: serde::Serializer,
{
    match set {
        Some(Set::One(value)) => value.serialize(serializer),
        Some(Set::Many(values)) => {
            let json_values: Vec<serde_json::Value> = values
                .iter()
                .map(|v| serde_json::to_value(v).unwrap())
                .collect();
            serde_json::Value::Array(vec![
                serde_json::Value::String(SET_HEADER.to_string()),
                serde_json::Value::Array(json_values),
            ])
            .serialize(serializer)
        }
        None => serde_json::Value::Null.serialize(serializer),
    }
}

pub fn deserialize_set<'de, D, T>(deserializer: D) -> Result<Option<Set<T>>, D::Error>
where
    D: Deserializer<'de>,
    T: serde::de::DeserializeOwned,
{
    let value = serde_json::Value::deserialize(deserializer)?;
    match value {
        serde_json::Value::Null => Ok(None),
        serde_json::Value::Array(arr) => {
            // array must contain 2 elements, first is a string "set", second is an array T[]
            if arr.len() != 2 {
                return Err(de::Error::custom("expected array with 2 elements"));
            }
            let set_header = arr[0]
                .as_str()
                .ok_or_else(|| de::Error::custom("expected string"))?;
            if set_header != SET_HEADER {
                return Err(de::Error::custom("expected 'set'"));
            }
            let values: Vec<T> = serde_json::from_value(arr[1].clone())
                .map_err(|e| de::Error::custom(e.to_string()))?;
            Ok(Some(Set::Many(values)))
        }
        // single value, value must match the type T
        _ => serde::Deserialize::deserialize(value)
            .map(Set::One)
            .map(Some)
            .map_err(|e| de::Error::custom(e.to_string())),
    }
}

// named uuid
// https://tools.ietf.org/html/rfc7047#section-3.1.2
// A 2-element JSON array that represents the UUID of a row inserted in a "insert" operation within the same transaction.
// the first element of the array must be the string "named-uuid",
// and the second element should be the <id> specified as the "uuid-name"
// for an "insert" operation within the same transaction.
// For example, if an "insert" operation within this transaction
// specifies a "uuid-name" of "myrow", the following <named-uuid> represents the UUID created by that operation:
// Example: ["uuid", "myrow"]

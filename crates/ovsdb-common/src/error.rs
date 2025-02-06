use serde::{Deserialize, Serialize};
// use serde_json;

// error
#[derive(Debug, Serialize, Deserialize)]
pub struct Error {
    error: String,
    details: Option<String>,
}

use ovsdb_schema::Schema;
use serde_json;
use std::fs;

pub fn load_schema_from_file(path: &str) -> Result<Schema, serde_json::Error> {
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");
    serde_json::from_str(&contents)
}

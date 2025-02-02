use crate::types::ColumnType;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct Schema {
    pub name: String,
    pub cksum: Option<String>,
    pub version: Option<String>,
    pub tables: HashMap<String, Table>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct Table {
    pub columns: HashMap<String, Column>,
    pub max_rows: Option<u64>,
    #[serde(default = "default_true")]
    pub is_root: bool,
    pub indexes: Option<Vec<Vec<String>>>,
}

fn default_true() -> bool {
    true
}

#[derive(Debug, Deserialize)]
pub struct Column {
    #[serde(rename = "type")]
    pub type_: ColumnType,
    #[serde(default)]
    pub ephemeral: bool,
    #[serde(default)]
    pub mutable: bool,
}

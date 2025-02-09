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
    max_rows: Option<u64>,
    #[serde(default = "default_true")]
    is_root: bool,
    indexes: Option<Vec<Vec<String>>>,
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

impl Table {
    pub fn is_root(&self) -> bool {
        self.is_root
    }

    pub fn iter_columns(&self) -> impl Iterator<Item = (&String, &Column)> {
        self.columns.iter()
    }

    pub fn index(&self) -> Option<&Vec<Vec<String>>> {
        self.indexes.as_ref()
    }

    pub fn get_max_rows(&self) -> Option<u64> {
        self.max_rows
    }

    pub fn has_index(&self) -> bool {
        self.indexes.is_some()
    }
}

impl Schema {
    pub fn iter_tables(&self) -> impl Iterator<Item = (&String, &Table)> {
        self.tables.iter()
    }

    pub fn get_table(&self, name: &str) -> Option<&Table> {
        self.tables.get(name)
    }


    pub fn table_names(&self) -> Vec<String> {
        self.tables.keys().cloned().collect()
    }

    pub fn table_count(&self) -> usize {
        self.tables.len()
    }

    pub fn has_table(&self, name: &str) -> bool {
        self.tables.contains_key(name)
    }
}

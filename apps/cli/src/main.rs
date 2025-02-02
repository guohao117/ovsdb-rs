use ovsdb_schema::Schema;
use std::env;
use std::fs;
use serde_json;
/// Loads an OVSDB schema from the given file path.
fn load_schema_from_file(path: &str) -> Schema {
    let data = fs::read_to_string(path)
        .expect("Failed to read the schema file");
    serde_json::from_str(&data)
        .expect("Failed to parse the schema")
}

/// Reads the schema file path from command-line arguments,
/// loads the schema, and prints the number of tables.
fn print_schema_table_count() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <schema_file>", args[0]);
        std::process::exit(1);
    }
    let schema = load_schema_from_file(&args[1]);
    println!("Number of tables: {}", schema.tables.len());
}

fn main() {
    print_schema_table_count();
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_schema_loading() {
        // Replace "test_schema.json" with the path to your test schema file.
        let schema = load_schema_from_file("./tests/ovn-nb.ovsschema");
        assert!(schema.tables.len() >= 0);
    }
}

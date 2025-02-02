use ovsdb_schema::Schema;
use std::env;
use std::fs;
use serde_json;
/// Loads an OVSDB schema from the given file path.
fn load_schema_from_file(path: &str) -> Result<Schema, serde_json::Error> {
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");
    serde_json::from_str(&contents)
}

/// Reads the schema file path from command-line arguments,
/// loads the schema, and prints the number of tables.
fn print_schema_table_count() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <schema_file>", args[0]);
        std::process::exit(1);
    }
    let schema = load_schema_from_file(&args[1]).expect("Failed to load schema");
    for (n, t) in schema.tables.iter() {
        if t.is_root() {
            println!("Root table: {}", n);
        }
        if let Some(indexes) = t.get_index_columns() {
            println!("Table {} Indexes: {:?}", n, indexes);
        }
    }
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
        assert!(schema.is_ok());
    }
}

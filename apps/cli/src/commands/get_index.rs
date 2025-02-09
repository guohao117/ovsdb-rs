use crate::utils::load_schema_from_file;

pub fn run_get_index(schema_file: &str, table: Option<&str>) {
    let schema = load_schema_from_file(schema_file).expect("Failed to load schema");

    if let Some(table_name) = table {
        if let Some(table_entry) = schema.get_table(table_name) {
            match table_entry.index() {
                Some(index) => println!("Index for table '{}': {:?}", table_name, index),
                None => println!("Table '{}' does not have any index.", table_name),
            }
        } else {
            println!("Table '{}' not found.", table_name);
        }
    } else {
        for (name, table_entry) in schema.iter_tables() {
            if let Some(index) = table_entry.index() {
                println!("Table '{}' has index: {:?}", name, index);
            }
        }
    }
}

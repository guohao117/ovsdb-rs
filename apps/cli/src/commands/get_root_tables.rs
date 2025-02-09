use crate::utils::load_schema_from_file;

pub fn run_get_root_tables(schema_file: &str) {
    let schema = load_schema_from_file(schema_file).expect("Failed to load schema");
    schema
        .iter_tables()
        .filter(|(_, t)| t.is_root())
        .for_each(|(n, _)| println!("Root table: {}", n));
    println!("Number of tables: {}", schema.tables.len());
}

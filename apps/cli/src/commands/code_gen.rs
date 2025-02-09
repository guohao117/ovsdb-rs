use crate::utils::load_schema_from_file;

pub fn run_code_gen(output_dir: &str, mod_name: &str, schema_file: &str) {
    let schema = load_schema_from_file(schema_file).expect("Failed to load schema");
    // ...code generation logic...
    println!("Generated code for module '{}' into directory '{}'", mod_name, output_dir);
}

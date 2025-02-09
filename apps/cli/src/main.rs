use ovsdb_schema::Schema;
use serde_json;
use std::fs;
use clap::Parser;
mod option;
mod commands;
mod utils;
use option::{CliOptions, Commands};

/// Loads an OVSDB schema from the given file path.
fn load_schema_from_file(path: &str) -> Result<Schema, serde_json::Error> {
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");
    serde_json::from_str(&contents)
}

/// Reads the CLI options and executes the corresponding subcommand.
fn run_cli() {
    let opts = CliOptions::parse();
    match opts.command {
        Some(Commands::GetRootTables { schema_file }) => {
            commands::get_root_tables::run_get_root_tables(&schema_file);
        },
        Some(Commands::CodeGen { output_dir, mod_name, schema_file }) => {
            commands::code_gen::run_code_gen(&output_dir, &mod_name, &schema_file);
        },
        Some(Commands::GetIndex { schema_file, table }) => {
            commands::get_index::run_get_index(&schema_file, table.as_deref());
        },
        _ => {
            eprintln!("A valid subcommand is required.");
        }
    }
}

fn main() {
    run_cli();
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

use clap::{Parser, Subcommand};

/// CLI options for ovsdb-rs CLI.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct CliOptions {
    /// Subcommand to execute.
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Prints the root tables.
    GetRootTables {
        /// OVSDB schema file path
        #[arg(short, long)]
        schema_file: String,
    },
    /// Generates code based on the schema.
    CodeGen {
        /// Output directory for generated files.
        #[arg(short, long)]
        output_dir: String,
        /// Module name for the generated code.
        #[arg(short, long)]
        mod_name: String,
        /// OVSDB schema file path.
        #[arg(short, long)]
        schema_file: String,
    },
    /// Retrieves index information for tables.
    GetIndex {
        /// OVSDB schema file path.
        #[arg(short, long)]
        schema_file: String,
        /// Specific table name to query; if omitted, list all tables with indexes.
        #[arg(short, long)]
        table: Option<String>,
    },
}

use std::path::PathBuf;

use clap::{Parser, Subcommand, ValueHint::FilePath};

#[derive(Parser)]
#[command(
    author,
    version,
    about = "A blazingly fast utility fot converting between file formats.",
    long_about = r#"
fiox, a blazingly fast, streaming-first file converter.

It supports JSON, TOML, CSV, and soon NDJSON.
Formats are detected automatically based on file content.
Output format is determined by file extension.

Examples:

    fiox convert data.json out.csv
    fiox validate broken.ndjson --verbose
    fiox convert big.csv big.json --verbose

"#
)]
pub struct FioxArgs {
    /// Flag for extra debug logging.
    #[arg(short, long)]
    pub verbose: bool,

    #[command(subcommand)]
    pub cmd: Commands,
}

/// Fiox subcommands
#[derive(Subcommand)]
pub enum Commands {
    /// Convert command that takes two positional arguments for input and output, takes one
    /// flag [`verbose`] which enables extra logs and backtraces.
    Convert {
        #[arg(short, long)]
        verbose: bool,

        #[arg(required = true, value_hint = FilePath)]
        input: PathBuf,

        #[arg(required = true, value_hint = FilePath)]
        output: PathBuf,
    },

    /// Validate command for file format validation with one positional argument for the file
    /// to be checked and a flag [`verbose`] which enables extra logs, backtraces and exact
    /// line errors.
    Validate {
        #[arg(required = true, value_hint = FilePath)]
        input: PathBuf,

        #[arg(short, long)]
        verbose: bool,
    },
}

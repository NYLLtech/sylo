mod init;
mod manifest;
mod scan;
mod utils;

use std::path::PathBuf;

use clap::{Parser, Subcommand};

use crate::init::init;
use crate::scan::scan;

const IGNORE_FILE_NAME: &'static str = ".syloignore";
const DIRECTORY_NAME: &'static str = ".sylo";
const MANIFEST_FILE_NAME: &'static str = "manifest.toml";

fn parse_key_value(string: &str) -> Result<(String, String), String> {
    let Some((key, value)) = string.split_once('=') else {
        return Err("expected KEY=VALUE".to_string());
    };

    if key.is_empty() {
        return Err("key cannot be empty".to_string());
    }

    if value.is_empty() {
        return Err("value cannot be empty".to_string());
    }

    Ok((key.to_string(), value.to_string()))
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Initialize a directory into a workspace.
    Init {
        /// Create an empty .syloignore file.
        #[arg(short, long, default_value_t = false)]
        ignore: bool,

        // Patterns in a KEY=VALUE format.
        #[arg(short, long, value_parser = parse_key_value)]
        patterns: Vec<(String, String)>,

        // Classifiers in a KEY=VALUE format.
        #[arg(short, long, value_parser = parse_key_value)]
        classifiers: Vec<(String, String)>,
    },

    /// Scan a workspace.
    Scan,

    /// Start the Sylo protocol server.
    Server,
}

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// The directory or workspace to operate on.
    #[arg(short, long)]
    path: Option<PathBuf>,

    #[command(subcommand)]
    command: Commands,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli: Cli = Cli::parse();
    let path: PathBuf = cli.path.unwrap_or(std::env::current_dir()?);

    match cli.command {
        Commands::Init {
            ignore,
            patterns,
            classifiers,
        } => init(path, ignore, patterns, classifiers),
        Commands::Scan => scan(path),
        Commands::Server => todo!(),
    }

    Ok(())
}

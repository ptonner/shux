use std::error::Error;
use std::process::Command;
// use std::fs;
// use std::path::PathBuf;
// use jupyter_client::Client;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about=None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Show available kernel connections
    Show {},
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Show {}) => {
            let jupyter_runtime_dir = Command::new("jupyter")
                .args(["--runtime-dir"])
                .output()
                .expect("Jupyter not found");
            println!(
                "runtime: {:?}",
                String::from_utf8(jupyter_runtime_dir.stdout)?
                    .strip_suffix("\n")
                    .unwrap()
            );
            // TODO: list connections at identified path
            // TODO: explore using `jupyter_client::find_connection_file`, patching broken linux
            // runtime spec
        }
        None => {}
    }

    Ok(())
}

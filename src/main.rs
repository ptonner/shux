use std::convert::Infallible;
use std::process::Command;
use std::{error::Error, str::FromStr};
// use std::fs;
use std::path::PathBuf;
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

fn jupyter_runtime_dir() -> Result<PathBuf, Infallible> {
    let jupyter_runtime_cmd = Command::new("jupyter")
        .args(["--runtime-dir"])
        .output()
        .expect("jupyter should be installed");
    let jupyter_runtime_output = String::from_utf8_lossy(&jupyter_runtime_cmd.stdout);
    PathBuf::from_str(
        jupyter_runtime_output
            .strip_suffix("\n")
            .expect("jupyter should report runtime directory"),
    )
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    match &cli.command {
        Some(Commands::Show {}) => {
            // TODO: add test that this lists for different runtime dirs (specify from environment
            // var)
            let jupyter_runtime_dir = jupyter_runtime_dir();
            println!("runtime: {:?}", jupyter_runtime_dir);
            // TODO: list connections at identified path
            // TODO: explore using `jupyter_client::find_connection_file`, patching broken linux
            // runtime spec
        }
        None => {}
    }

    Ok(())
}

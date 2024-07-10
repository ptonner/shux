use std::fs;
use std::{error::Error, path::PathBuf};

use clap::Parser;

pub mod paths;
pub mod select;

#[derive(Parser)]
#[command(version, about, long_about=None)]
struct Cli {
    connection_file: PathBuf,
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    let data = fs::read(cli.connection_file)?;
    let data = String::from_utf8_lossy(&data).to_owned();
    let connection_cfg: serde_json::Value = serde_json::from_str(&data)?;
    println!("{:}", connection_cfg);
    Ok(())
}

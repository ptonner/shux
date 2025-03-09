use std::error::Error;
use std::path::PathBuf;

use clap::{Parser, Subcommand};

mod conf;
mod select;
mod send;
mod ui;
mod watch;

#[derive(Parser)]
#[command(version, about, long_about=None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    /// Path to configuration file
    #[arg(short, long)]
    config: Option<PathBuf>,

    /// Path to kernel connection file
    #[arg(short, long)]
    kernel: Option<PathBuf>,
}

#[derive(Subcommand)]
enum Commands {
    /// Recieve input from stdin, send as command kernel
    Send { command: String },
    /// Select a default kernel
    Select { kernel: Option<PathBuf> },
    /// Watch a kernel for messages
    Watch {},
    /// Run shux UI
    UI {},
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    // setup config
    if let Some(ref conf) = cli.config {
        conf::set_config_env(&conf);
    }
    if let Some(ref kernel) = cli.config {
        conf::set_kernel_env(&kernel);
    }
    let mut conf = conf::Config::load();

    match &cli.command {
        Some(Commands::Send { command }) => {
            send::run(&conf, command.into());
        }
        Some(Commands::Select { kernel }) => {
            select::select_kernel(&mut conf, kernel);
        }
        Some(Commands::Watch {}) => {
            watch::run(&conf)?;
        }
        Some(Commands::UI {}) => {
            ui::ui(&conf)?;
        }
        None => {}
    }

    Ok(())
}

use std::convert::Infallible;
use std::io;
use std::path::PathBuf;
use std::process::Command;
use std::{error::Error, str::FromStr};

use clap::{Parser, Subcommand};
use ratatui::{
    crossterm::event::{self, KeyCode, KeyEventKind},
    style::Stylize,
    widgets::Paragraph,
    DefaultTerminal,
};

mod conf;
mod select;
mod send;
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
    /// Show available kernel connections
    List {},
    Watch {},
    UI {},
    /// Recieve input from stdin, send as command kernel
    Send {
        command: String,
    },
    Select {
        kernel: Option<PathBuf>,
    },
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

fn run(mut terminal: DefaultTerminal) -> io::Result<()> {
    loop {
        terminal.draw(|frame| {
            let greeting = Paragraph::new("Hello Ratatui! (press 'q' to quit)")
                .white()
                .on_blue();
            frame.render_widget(greeting, frame.area());
        })?;

        if let event::Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                return Ok(());
            }
        }
    }
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
        Some(Commands::List {}) => {
            // TODO: add test that this lists for different runtime dirs (specify from environment
            // var)
            let jupyter_runtime_dir = jupyter_runtime_dir();
            println!("runtime: {:?}", jupyter_runtime_dir);
            // TODO: list connections at identified path
            // TODO: explore using `jupyter_client::find_connection_file`, patching broken linux
            // runtime spec
        }
        Some(Commands::UI {}) => {
            let mut terminal = ratatui::init();
            terminal.clear()?;
            let app_result = run(terminal);
            ratatui::restore();
            println!("{:?}", app_result);
        }
        Some(Commands::Send { command }) => {
            send::run(&conf, command.into());
        }
        Some(Commands::Select { kernel }) => {
            select::select_kernel(&mut conf, kernel);
        }
        Some(Commands::Watch {}) => {
            watch::run(&conf)?;
        }
        None => {}
    }

    Ok(())
}

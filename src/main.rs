use std::convert::Infallible;
use std::fs::File;
use std::io;
use std::path::PathBuf;
use std::process::Command;
use std::thread;
use std::time::Duration;
use std::{error::Error, str::FromStr};

use clap::{Parser, Subcommand};
use jupyter_client::Client;
use ratatui::{
    crossterm::event::{self, KeyCode, KeyEventKind},
    style::Stylize,
    widgets::Paragraph,
    DefaultTerminal,
};

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
    Watch {
        kernel: PathBuf,
    },
    UI {},
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
        Some(Commands::UI {}) => {
            let mut terminal = ratatui::init();
            terminal.clear()?;
            let app_result = run(terminal);
            ratatui::restore();
            println!("{:?}", app_result);
        }
        Some(Commands::Watch { kernel }) => {
            println!("watching: {:?}", kernel);
            match File::open(kernel) {
                Ok(reader) => {
                    match Client::from_reader(reader) {
                        Ok(client) => {
                            // iopub
                            let receiver = client.iopub_subscribe()?;
                            thread::spawn(move || {
                                for msg in receiver {
                                    println!("Received message from kernel: {:#?}", msg);
                                }
                            });

                            // wait
                            loop {
                                thread::sleep(Duration::from_secs(2));
                            }
                        }
                        Err(err) => println!("Client error: {:?}", err),
                    }
                }
                Err(err) => println!("File error: {:?}", err),
            }
        }
        None => {}
    }

    Ok(())
}

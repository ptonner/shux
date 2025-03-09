use jupyter_client::commands::Command;
use jupyter_client::Client;
use std::collections::HashMap;
use std::fs::File;
use std::path::PathBuf;

use crate::conf::Config;

pub fn run(command: String, kernel: Option<PathBuf>) {
    let kernel = kernel.or_else(|| Config::load().kernel);
    if let Some(kernel) = kernel {
        println!("Using kernel: {:?}", kernel);
        match File::open(kernel) {
            Ok(reader) => match Client::from_reader(reader) {
                Ok(client) => {
                    if let Ok(resp) = client.send_shell_command(Command::Execute {
                        code: command,
                        silent: false,
                        store_history: true,
                        user_expressions: HashMap::new(),
                        allow_stdin: true,
                        stop_on_error: true,
                    }) {
                        dbg!("Response: {:?}", resp);
                    };
                }
                Err(err) => println!("Client error: {:?}", err),
            },
            Err(err) => println!("File error: {:?}", err),
        }
    } else {
        println!("No kernel found, please select a kernel");
        return;
    }
}

use serde::{Deserialize, Serialize};
use std::fs;
use std::{error::Error, path::PathBuf};
// use tokio::{main, test};
use zeromq::{Socket, SocketRecv, SocketSend};

use clap::Parser;

pub mod paths;
pub mod select;

#[derive(Parser)]
#[command(version, about, long_about=None)]
struct Cli {
    connection_file: PathBuf,
}

#[derive(Serialize, Deserialize, Debug)]
struct ConnectionFile {
    control_port: u32,
    shell_port: u32,
    stdin_port: u32,
    hb_port: u32,
    iopub_port: u32,
    key: String,
    transport: String,
    signature_scheme: String,
    ip: String,
}

fn load_connection_file(path: PathBuf) -> Result<ConnectionFile, Box<dyn Error>> {
    let data = fs::read(path)?;
    let data = String::from_utf8_lossy(&data).to_owned();
    Ok(serde_json::from_str(&data)?)
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    // let data = fs::read(cli.connection_file)?;
    // let data = String::from_utf8_lossy(&data).to_owned();
    let connection_cfg = load_connection_file(cli.connection_file)?;
    println!("{:?}", connection_cfg);
    Ok(())
}

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn Error>> {
//     let mut socket = zeromq::ReqSocket::new();
//     socket
//         .connect("tcp://127.0.0.1:5555")
//         .await
//         .expect("Failed to connect");
//     println!("Connected to server");
//
//     for _ in 0..10u64 {
//         socket.send("Hello".into()).await?;
//         let repl = socket.recv().await?;
//         dbg!(repl);
//     }
//     Ok(())
// }

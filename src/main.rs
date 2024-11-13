use serde::{Deserialize, Serialize};
use std::fs;
use std::time::Duration;
use std::{error::Error, path::PathBuf};
use tokio::time::timeout;
// use tokio::{main, test};
use zeromq::{ReqSocket, Socket, SocketRecv, SocketSend};

use clap::Parser;

pub mod paths;
pub mod select;

#[derive(Parser)]
#[command(version, about, long_about=None)]
struct Cli {
    connection_file: PathBuf,
}

enum JupyterSocket {
    Control,
    Shell,
    Stdin,
    HeartBeat,
    IOPub,
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

impl ConnectionFile {
    fn address(&self, socket: JupyterSocket) -> String {
        let base = format!("{}://{}", self.transport, self.ip);
        match socket {
            JupyterSocket::Control => format!("{}:{}", base, self.control_port),
            JupyterSocket::Shell => format!("{}:{}", base, self.shell_port),
            JupyterSocket::Stdin => format!("{}:{}", base, self.stdin_port),
            JupyterSocket::HeartBeat => format!("{}:{}", base, self.hb_port),
            JupyterSocket::IOPub => format!("{}:{}", base, self.iopub_port),
        }
    }
}

fn load_connection_file(path: PathBuf) -> Result<ConnectionFile, Box<dyn Error>> {
    let data = fs::read(path)?;
    let data = String::from_utf8_lossy(&data).to_owned();
    Ok(serde_json::from_str(&data)?)
}

async fn run_heartbeat(mut socket: ReqSocket) -> Result<(), Box<dyn Error>> {
    loop {
        if let Err(e) = timeout(Duration::from_secs(5), socket.send("hb".into())).await? {
            break Err(Box::new(e));
        }
        let msg = match timeout(Duration::from_secs(5), socket.recv()).await? {
            Ok(msg) => msg,
            Err(e) => break Err(Box::new(e)),
        };
        dbg!(msg);
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    let connection_cfg = load_connection_file(cli.connection_file)?;
    // println!("{:?}", connection_cfg.address(JupyterSocket::HeartBeat));
    let mut socket = ReqSocket::new();
    socket
        .connect(&connection_cfg.address(JupyterSocket::HeartBeat))
        .await
        .expect("Heart beat should be available");
    if let Err(e) = run_heartbeat(socket).await {
        println!("{:}", e)
    };
    // for _ in 0..10u64 {
    //     socket.send("Hello".into()).await?;
    //     let repl = socket.recv().await?;
    //     dbg!(repl);
    // }
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

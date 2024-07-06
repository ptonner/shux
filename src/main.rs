use std::error::Error;
use std::fs;
use std::path::PathBuf;
use zeromq::{Socket, SocketRecv, SocketSend, ZmqError};

use clap::Parser;

// async fn handle_hb(connection: &mut zeromq::ReqSocket) -> Result<(), ()> {
//     loop {
//         connection
//             .send("heartbeat".into())
//             // .send(zeromq::ZmqMessage::from(b"ping".to_vec()))
//             .await?;
//         match connection.recv().await {
//             Ok(_) => {}
//             Err(e) => return Ok(()),
//         }
//     }
// }

#[derive(Parser)]
struct Cli {
    /// kernelspec file to make connection
    kernelspec: PathBuf,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    let data = fs::read_to_string(cli.kernelspec)?;
    let spec: serde_json::Value = serde_json::from_str(&data)?;
    // println!("kspec: {:?}", spec);

    // println!("{}://{}:{}", spec["transport"], spec["ip"], spec["hb_port"]);
    let address = format!(
        "{}://{}:{}",
        spec["transport"].as_str().unwrap(),
        spec["ip"].as_str().unwrap(),
        spec["hb_port"]
    );
    println!("Connecting to heart beat at: {:}", address);

    let mut socket = zeromq::ReqSocket::new();
    socket.connect(&address).await.expect("Failed to connect");
    println!("Connected to server");

    // handle_hb(&mut socket).await?

    // for _ in 0..10u64 {
    loop {
        socket.send("Hello".into()).await?;
        let repl = socket.recv().await?;
        dbg!(repl);
    }

    // Ok(())
}

use std::{error::Error, fs::File, path::PathBuf, time::Duration};

use jupyter_client::Client;

pub fn run(kernel: &PathBuf) -> Result<(), Box<dyn Error>> {
    println!("watching: {:?}", kernel);
    let reader = File::open(kernel)?;
    let client = Client::from_reader(reader)?;
    let receiver = client.iopub_subscribe()?;
    let heartbeat = client.heartbeat_every(Duration::from_secs(2))?;

    loop {
        if let Ok(msg) = receiver.recv_timeout(Duration::from_secs(2)) {
            println!("{:#?}", msg);
        }

        if heartbeat.try_recv().is_err() {
            println!("No heartbeat");
            break;
        }
    }
    Ok(())
}

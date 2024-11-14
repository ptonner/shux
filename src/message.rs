use hmac::{Hmac, Mac};
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use zeromq::ZmqMessage;

type Digester = Hmac<Sha256>;

///The connection file of a jupyter kernel
#[derive(Serialize, Deserialize, Debug)]
struct MessageHeader {
    msg_id: String,
    session: String,
    username: String,
    date: String,
    msg_type: String,
    version: String,
}

fn decode_blocks(message: ZmqMessage) -> Vec<String> {
    message
        .into_vec()
        .iter()
        .map(|b| String::from_utf8(b.to_vec()).unwrap_or_default())
        .collect()
}

pub fn decode_message(message: ZmqMessage) {
    match decode_blocks(message).as_slice() {
        [id, delim, sig, header, parent_header, metadata, content, ..] if delim == "<IDS|MSG>" => {
            let mut mac = Digester::new_from_slice(b"my secret and secure key")
                .expect("HMAC can take key of any size");
            mac.update(b"input message");
            dbg!(id);
        }
        _ => (),
    }
    // if let [id, delim, sig, header, parent_header, metadata, content, ..] =
    //     decode_blocks(message).as_slice()
    //     // message.into_vec().as_slice()
    // {
    //     if let Ok(delim) = String::from_utf8(delim.to_vec()).
    //     {}
    //     dbg!(id);
    // }
    // if let Some(header) = message.get(4) {
    //     dbg!(header);
    // }
    // if let [id, delim, sig, header, parent_header, metadata, content, ..] = message.into_vec()[..] {
    //     let header: MessageHeader =
    //         serde_json::from_str(String::from_utf8(header.to_vec()).unwrap().as_str()).unwrap();
    //     dbg!(header);
    // }
}

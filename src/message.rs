///The Jupyter Message Protocol
use bytes::Bytes;
use hex::decode;
use hmac::{Hmac, Mac};
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use zeromq::ZmqMessage;

pub type Digester = Hmac<Sha256>;

///The header of a jupyter kernel message
#[derive(Serialize, Deserialize, Debug)]
struct MessageHeader {
    msg_id: String,
    session: String,
    username: String,
    date: String,
    msg_type: String,
    version: String,
}

fn valid_sigature(
    mut digester: Digester,
    sig: &Bytes,
    header: &Bytes,
    parent_header: &Bytes,
    metadata: &Bytes,
    content: &Bytes,
) -> bool {
    digester.update(header);
    digester.update(parent_header);
    digester.update(metadata);
    digester.update(content);
    if let Ok(hsig) = decode(sig) {
        match digester.verify_slice(hsig.as_slice()) {
            Ok(()) => true,
            Err(_) => false,
        }
    } else {
        false
    }
}

pub fn decode_message(digester: Digester, message: ZmqMessage) {
    match message.into_vec().as_slice() {
        [id, delim, sig, header, parent_header, metadata, content, ..]
            if valid_sigature(digester, sig, header, parent_header, metadata, content) =>
        {
            println!("valid signature");
            ()
        }
        [..] => (),
    }
}

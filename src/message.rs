///The Jupyter Message Protocol
use bytes::Bytes;
use hex::decode;
use hmac::{Hmac, Mac};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sha2::Sha256;
use zeromq::ZmqMessage;

pub type Digester = Hmac<Sha256>;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
enum MessageType {
    Status,
    Stream,
    DisplayData,
    UpdateDisplayData,
    ExecuteInput,
    ExecuteResult,
    ExecuteRequest,
    Error,
    ClearOutput,
    DebugEvent,
}

///The header of a jupyter kernel message
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
struct MessageHeader {
    msg_id: String,
    session: String,
    username: String,
    date: String,
    // msg_type: String,
    msg_type: MessageType,
    version: String,
}

// Content

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
struct Stream {
    name: String,
    text: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
struct DisplayData {
    source: String,
    data: Value,
    metadata: Value,
    transient: Value,
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
            if delim.to_vec() == b"<IDS|MSG>"
                && valid_sigature(digester, sig, header, parent_header, metadata, content) =>
        {
            let fields = (
                serde_json::from_slice::<MessageHeader>(header),
                serde_json::from_slice::<MessageHeader>(parent_header),
            );
            if let (Ok(header), Ok(parent_header)) = fields {
                dbg!(id);
                dbg!(&header);
                dbg!(parent_header);
                let content = match &header.msg_type {
                    MessageType::DisplayData => serde_json::from_slice::<DisplayData>(content),
                    _ => todo!(),
                };
            } else {
                dbg!(fields);
            }
        }
        [..] => (),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_message_header() {
        let json_data = "{\"msg_id\": \"44020b88-b2ac6dedcb4b258bf630b565_6108_328\", \"msg_type\": \"status\", \"username\": \"ptonner\", \"session\": \"44020b88-b2ac6dedcb4b258bf630b565\", \"date\": \"2024-11-15T01:46:35.992926Z\", \"version\": \"5.3\"}";
        let my_enum: MessageHeader =
            serde_json::from_str(json_data).expect("Failed to deserialize");
        assert_eq!(
            my_enum,
            MessageHeader {
                msg_id: "44020b88-b2ac6dedcb4b258bf630b565_6108_328".into(),
                session: "44020b88-b2ac6dedcb4b258bf630b565".into(),
                username: "ptonner".into(),
                date: "2024-11-15T01:46:35.992926Z".into(),
                msg_type: MessageType::Status,
                version: "5.3".into()
            }
        )
    }
}

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    /// MAJOR.MINOR.PATCH
    /// See https://semver.org/
    pub version: String,
    /// Type of message
    #[serde(flatten)]
    pub kind: MessageType,
    pub party: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum MessageType {
    /// Sent as Vec<u8> for easy async, but handling the payload as &[u8] will be more efficient
    ShowImage {
        #[serde(with = "serde_bytes")]
        bytes: Vec<u8>,
    },
    /// Sent as Vec<u8> for easy async, but handling the payload as &[u8] will be more efficient
    ShowVideo {
        #[serde(with = "serde_bytes")]
        bytes: Vec<u8>,
    },
    Clear,
}

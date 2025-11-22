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
        options: DisplayOptions,
    },
    /// Sent as Vec<u8> for easy async, but handling the payload as &[u8] will be more efficient
    ShowVideo {
        #[serde(with = "serde_bytes")]
        bytes: Vec<u8>,
        options: DisplayOptions,
    },
    Clear,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DisplayOptions {
    /// Amount of time in milliseconds the media should stay on screen
    pub timeout_ms: u32,
}

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    /// MAJOR.MINOR.PATCH
    /// See https://semver.org/
    pub version: String,
    pub party: Option<String>,

    /// Type of message
    #[serde(flatten)]
    pub kind: MessageType,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum MessageType {
    // TODO
    JoinParty,
    ShowMedia {
        overlays: Vec<Overlay>,
        options: DisplayOptions,
    },
    Clear,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Overlay {
    /// Amount of time in milliseconds the media should stay on screen
    /// Sent as Vec<u8> for easy async, but handling the payload as &[u8] will be more efficient
    #[serde(with = "serde_bytes")]
    pub bytes: Vec<u8>,

    pub x: i32,
    pub y: i32,

    /// z-order for composition (0 = behind, high = front)
    pub z_index: u32,
    // Amount of time in milliseconds the overlay should stay on screen
    // pub timeout_ms: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DisplayOptions {
    /// Amount of time in milliseconds the overlay should stay on screen
    pub timeout_ms: u32,
}

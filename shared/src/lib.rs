use serde::{Deserialize, Serialize};

/// Full message wrapper exchanged between all components
#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    /// MAJOR.MINOR.PATCH following semver
    pub version: String,
    pub party: Party,

    /// The actual payload
    #[serde(flatten)]
    pub kind: MessageType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Party {
    pub id: String,
    pub name: String,
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
    /// FOR DEBUG
    /// Removes anything shown on the splash screen
    Clear,
}

#[derive(Serialize, Deserialize, Debug)]
// TODO : Add a timeout_ms, an overlay should be able to last not as long as the overall media
pub enum Overlay {
    Text {
        text: String,
        size: u32,
        color: [u8; 4],
        x: i32,
        y: i32,
        z_index: u32,
    },
    Image {
        // Sent as Vec<u8> for easy async, but handling the payload as &[u8] will be more efficient
        /// raw PNG/JPEG/WebP/etc.
        #[serde(with = "serde_bytes")]
        bytes: Vec<u8>,
        x: i32,
        y: i32,
        /// z-order for composition (0 = behind, high = front)
        z_index: u32,
    },

    AnimatedImage {
        #[serde(with = "serde_bytes")]
        bytes: Vec<u8>,
        x: i32,
        y: i32,
        z_index: u32,
    },
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DisplayOptions {
    /// Amount of time in milliseconds the overlay should stay on screen
    pub timeout_ms: u32,
}

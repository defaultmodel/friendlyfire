use std::fs;

use friendlyfire_shared_lib::{Message, MessageType};

/// Fake client that sends mock messages for now
pub struct WSClient {
    sent: bool,
    msg_bytes: Vec<u8>,
}

impl WSClient {
    pub fn connect(_url: &str) -> Self {
        // encode bonk.png as base64 and wrap in a ShowImage message
        let img_bytes = fs::read("bonk.png").expect("bonk.png not found, sending empty image");

        let msg = Message {
            version: "1.0.0".to_string(),
            kind: MessageType::ShowImage { bytes: img_bytes },
            party: Some("test-party".to_string()),
        };

        Self {
            sent: false,
            msg_bytes: rmp_serde::to_vec(&msg).unwrap(),
        }
    }

    /// Returns Some(message) the first time, None afterwards
    pub fn recv(&mut self) -> Option<&[u8]> {
        if !self.sent {
            self.sent = true;
            Some(&self.msg_bytes)
        } else {
            None
        }
    }
}

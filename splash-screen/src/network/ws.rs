use std::fs;

use friendlyfire_shared_lib::{Message, MessageType};

/// Fake client that sends mock messages for now
pub struct WSClient {
    sent: bool,
    msg_bytes: Vec<u8>,
}

impl WSClient {
    pub fn connect(_url: &str) -> Self {
        todo!()
    }

    /// Returns Some(message) the first time, None afterwards
    pub fn recv(&mut self) -> Option<&[u8]> {
        todo!()
    }
}

use std::io::Cursor;

use image::ImageReader;

use crate::{frame::Frame, overlay::Overlay};

pub struct ImageOverlay {
    z_index: u32,
    pub frame: Frame,
}

impl ImageOverlay {
    pub fn from_bytes(bytes: &[u8], left: i32, top: i32, z_index: u32) -> Self {
        let rgba = ImageReader::new(Cursor::new(bytes))
            .with_guessed_format()
            .ok()
            .unwrap()
            .decode()
            .ok()
            .unwrap()
            .to_rgba8();

        let (width, height) = rgba.dimensions();
        let frame = Frame::from_bytes(left, top, width, height, &rgba, 0);

        Self { z_index, frame }
    }
}

impl Overlay for ImageOverlay {
    fn z_index(&self) -> u32 {
        self.z_index
    }

    fn draw(&self, target: &mut Frame, _timestamp_ms: u64) {
        target.blit(
            self.frame.left as u32,
            self.frame.top as u32,
            self.frame.width,
            self.frame.height,
            &self.frame.buffer,
        );
    }

    fn time_to_next_frame_ms(&self, _timestamp_ms: u64) -> Option<u64> {
        None
    }
}

use std::io::Cursor;

use image::{AnimationDecoder, codecs::gif};

use crate::{frame::Frame, overlay::Overlay};

pub struct AnimatedOverlay {
    pub frames: Vec<Frame>,
    pub z_index: u32,
    pub start_time_ms: u64, // when the animation started
}

impl AnimatedOverlay {
    pub fn from_bytes(bytes: &[u8], x: i32, y: i32, z_index: u32, start_time_ms: u64) -> Self {
        let cursor = Cursor::new(bytes);
        let decoder = gif::GifDecoder::new(cursor).ok().unwrap();
        let frames_iter = decoder.into_frames();

        let frames = frames_iter
            .map(|frame| {
                let frame = frame.unwrap();
                let delay_ms = {
                    let (numerator, denominator) = frame.delay().numer_denom_ms();
                    numerator / denominator
                };
                let rgba = frame.into_buffer();
                let (width, height) = rgba.dimensions();
                Frame::from_bytes(x, y, width, height, &rgba, delay_ms)
            })
            .collect();

        Self {
            frames,
            z_index,
            start_time_ms,
        }
    }

    /// Returns the index of the frame to draw for a given timestamp
    fn current_frame_index(&self, timestamp_ms: u64) -> usize {
        if self.frames.is_empty() {
            return 0;
        }

        // compute total animation duration

        let total_duration: u64 = self.frames.iter().map(|f| f.delay_ms as u64).sum();
        let elapsed = timestamp_ms.saturating_sub(self.start_time_ms);

        if total_duration == 0 {
            return 0; // static single-frame
        }

        // loop by default
        let mut time_in_cycle = elapsed % total_duration;

        // find the frame corresponding to the elapsed time
        for (i, frame) in self.frames.iter().enumerate() {
            if time_in_cycle < frame.delay_ms as u64 {
                return i;
            }
            time_in_cycle -= frame.delay_ms as u64;
        }

        // fallback to last frame
        self.frames.len() - 1
    }
}

impl Overlay for AnimatedOverlay {
    fn z_index(&self) -> i32 {
        self.z_index as i32
    }

    fn draw(&self, target: &mut Frame, timestamp_ms: u64) {
        if self.frames.is_empty() {
            return;
        }

        let idx = self.current_frame_index(timestamp_ms);
        let frame = &self.frames[idx];

        target.blit(
            frame.left as u32,
            frame.top as u32,
            frame.width,
            frame.height,
            &frame.buffer,
        );
    }
}

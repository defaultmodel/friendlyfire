use std::io::Cursor;

use image::{AnimationDecoder, codecs::gif};

use crate::{frame::Frame, overlay::Overlay};

pub struct AnimatedOverlay {
    pub frames: Vec<Frame>,
    z_index: u32,
    start_time_ms: u128, // when the animation started
}

impl AnimatedOverlay {
    pub fn from_bytes(bytes: &[u8], x: i32, y: i32, z_index: u32, start_time_ms: u128) -> Self {
        let cursor = Cursor::new(bytes);
        let decoder = gif::GifDecoder::new(cursor).ok().unwrap();
        let frames_iter = decoder.into_frames();

        let frames = frames_iter
            .map(|frame| {
                let frame = frame.unwrap();

                // delay in `image` is reprenseted by a fraction, we resolve the fraction
                let delay_ms = {
                    let (numerator, denominator) = frame.delay().numer_denom_ms();
                    numerator as u128 / denominator as u128
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

    fn current_frame_index(&self, timestamp_ms: u128) -> usize {
        if self.frames.is_empty() {
            return 0;
        }

        // compute total animation duration
        let total_duration: u128 = self.frames.iter().map(|f| f.delay_ms).sum();
        let elapsed = timestamp_ms.saturating_sub(self.start_time_ms);

        if total_duration == 0 {
            return 0; // static single-frame
        }

        // loop by default
        let mut time_in_cycle = elapsed % total_duration;

        // find the frame corresponding to the elapsed time
        for (i, frame) in self.frames.iter().enumerate() {
            if time_in_cycle < frame.delay_ms {
                return i;
            }
            time_in_cycle -= frame.delay_ms;
        }

        // fallback to last frame
        self.frames.len() - 1
    }

    /// Compute how many ms remain until the end of the current frame.
    fn time_remaining_on_current_frame(&self, timestamp_ms: u128) -> Option<u128> {
        if self.frames.is_empty() {
            return None;
        }

        let total_duration: u128 = self.frames.iter().map(|f| f.delay_ms).sum();
        if total_duration == 0 {
            return None;
        }

        let elapsed = timestamp_ms.saturating_sub(self.start_time_ms);
        let mut time_in_cycle = elapsed % total_duration;

        for frame in &self.frames {
            let dur = frame.delay_ms;
            if time_in_cycle < dur {
                return Some(dur - time_in_cycle);
            }
            time_in_cycle -= dur;
        }

        // should not reach here, but fallback to first frame duration
        Some(self.frames[0].delay_ms)
    }
}

impl Overlay for AnimatedOverlay {
    fn z_index(&self) -> u32 {
        self.z_index
    }

    fn draw(&self, target: &mut Frame, timestamp_ms: u128) {
        if self.frames.is_empty() {
            return;
        }

        let idx = self.current_frame_index(timestamp_ms);
        let frame = &self.frames[idx];

        target.blit(
            frame.offset_left as u32,
            frame.offset_top as u32,
            frame.width,
            frame.height,
            &frame.buffer,
        );
    }

    fn time_to_next_frame_ms(&self, timestamp_ms: u128) -> Option<u128> {
        self.time_remaining_on_current_frame(timestamp_ms)
    }
}

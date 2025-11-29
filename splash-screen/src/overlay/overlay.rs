use crate::frame::Frame;

pub trait Overlay {
    /// z-order for composition (0 = behind, high = front)
    fn z_index(&self) -> i32;

    /// Draws the overlay into the pixel buffer of the Frame
    fn draw(&self, frame: &mut Frame, timestamp_ms: u64);

    /// Time in milliseconds until this overlay wants the next frame change.
    /// - Return `None` if this overlay does not have a timed next frame (static image).
    /// - Return `Some(remaining_ms)` where `remaining_ms` is >= 0.
    fn time_to_next_frame_ms(&self, timestamp_ms: u64) -> Option<u64>;
}

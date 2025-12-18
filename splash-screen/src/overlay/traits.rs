use crate::frame::Frame;

/// Specifies an element that can be composited onto a `Frame`.
pub trait Overlay {
    /// Z-order for composition (0 = back, high = front).
    fn z_index(&self) -> u32;

    /// Draw the `Overlay` into the given `Frame` for the specified timestamp.
    fn draw(&self, frame: &mut Frame, timestamp_ms: u64);

    /// Time in milliseconds until this overlay wants the next `Frame`.
    /// - Return `None` if this overlay does not have a timed next frame (static image).
    /// - Return `Some(remaining_ms)` where `remaining_ms` is >= 0.
    fn time_to_next_frame_ms(&self, timestamp_ms: u64) -> Option<u64>;
}

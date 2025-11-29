use crate::frame::Frame;

pub trait Overlay {
    /// z-order for composition (0 = behind, high = front)
    fn z_index(&self) -> i32;

    /// Draws the overlay into the pixel buffer of the Frame
    fn draw(&self, frame: &mut Frame, timestamp_ms: u64);
}

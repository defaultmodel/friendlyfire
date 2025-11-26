use crate::frame::Frame;

pub enum DecodedMedia {
    Static(Frame),
    Animated(Vec<Frame>),
    Video(Box<dyn VideoStream>),
}

pub trait VideoStream {
    /// Returns the next frame, or None if finished
    fn next_frame(&mut self) -> Option<Frame>;

    /// Returns the frame count, if known
    fn frame_count(&self) -> Option<usize>;
}

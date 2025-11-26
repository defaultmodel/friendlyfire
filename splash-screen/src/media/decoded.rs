use crate::frame::Frame;

pub enum DecodedMedia {
    Static(Frame),
    Animated(Vec<Frame>),
}

use crate::frame::Frame;

pub enum Media {
    Static(Frame),
    Animated(Vec<Frame>),
}

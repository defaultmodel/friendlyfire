use crate::{frame::Frame, overlay::Overlay};

pub struct Compositor {
    /// The main output frame (canvas)
    pub canvas: Frame,
    /// Registered overlays (static or animated)
    pub overlays: Vec<Box<dyn Overlay>>,
}

impl Compositor {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            canvas: Frame::new(0, 0, width, height, 0),
            overlays: Vec::new(),
        }
    }

    pub fn add_overlay(&mut self, overlay: Box<dyn Overlay>) {
        self.overlays.push(overlay);
    }

    pub fn render(&mut self, timestamp_ms: u64) -> &Frame {
        // clear canvas
        self.canvas.clear();

        // sort overlays by z_index ascending
        self.overlays.sort_by_key(|o| o.z_index());

        // draw each overlay
        for overlay in &self.overlays {
            overlay.draw(&mut self.canvas, timestamp_ms);
        }

        &self.canvas
    }
}

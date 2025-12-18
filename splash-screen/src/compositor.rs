use crate::{frame::Frame, overlay::Overlay};

/// Central composition engine responsible for producing the "final `Frame`" from a bunch of `Overlay`.
///
/// It's job is to order the multiple `Overlay` given to him by their `Overlay.z_index()` and draw them in order.
pub struct Compositor {
    /// The main output frame (canvas).
    ///
    /// The origin point of the canvas is the same as `Overlay`, i.e. it follow the CSSOM standard.
    /// See https://developer.mozilla.org/en-US/docs/Web/API/CSSOM_view_API/Coordinate_systems
    pub canvas: Frame,
    /// Registered overlays (static or animated).
    pub overlays: Vec<Box<dyn Overlay>>,
}

impl Compositor {
    /// Create a new compositor with a fixed canvas size.
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            canvas: Frame::new(0, 0, width, height, 0),
            overlays: Vec::new(),
        }
    }

    /// Register a new overlay that will be composited onto the `self.canvas`.
    pub fn add_overlay(&mut self, overlay: Box<dyn Overlay>) {
        self.overlays.push(overlay);
    }

    /// Render the `self.canvas` for the given timestamp.
    pub fn render(&mut self, timestamp_ms: u128) -> &Frame {
        self.canvas.clear();

        self.overlays.sort_by_key(|o| o.z_index());

        for overlay in &self.overlays {
            overlay.draw(&mut self.canvas, timestamp_ms);
        }

        &self.canvas
    }

    /// Return the earliest time any `Overlay` wants its next `Frame` to be shown.
    ///
    /// Returning `None` indicates that no overlay require time-based updates (static content).
    pub fn time_until_next_frame_ms(&self, timestamp_ms: u128) -> Option<u128> {
        self.overlays
            .iter()
            .filter_map(|o| o.time_to_next_frame_ms(timestamp_ms))
            .min()
    }
}

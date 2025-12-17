use serde::{Deserialize, Serialize};

/// Media that can be decoded/rasterized and composited onto a `Frame`.
#[derive(Serialize, Deserialize, Debug)]
// Self-contained because they go through a different canal than traditionnal messages because of their size.
// TODO : Add a timeout_ms, an overlay should be able to last not as long as the overall media.
pub enum Overlay {
    Text {
        /// UTF-8 text content to render.
        text: String,

        /// Font size in logical pixels.
        size: u32,

        /// RGBA color (0–255 per channel).
        color: [u8; 4],

        /// Horizontal offset from the left edge. We are using the top-left corner as the origin as seen in CSSOM.
        /// See https://developer.mozilla.org/en-US/docs/Web/API/CSSOM_view_API/Coordinate_systems
        ///
        /// The offset can be negative, so that image may appear cropped out of the `Frame`
        offset_left: i32,

        /// Vertical offset from the top edge. We are using the top-left corner as the origin as seen in CSSOM.
        /// See https://developer.mozilla.org/en-US/docs/Web/API/CSSOM_view_API/Coordinate_systems
        ///
        /// The offset can be negative, so that image may appear cropped out of the `Frame`
        offset_top: i32,

        /// Z-order for composition (0 = back, high = front).
        z_index: u32,
    },
    Image {
        /// Raw encoded image data (PNG / JPEG / WebP / etc).
        /// This data is decoded via the `image` crate, thus any variant shown [here](https://docs.rs/image/latest/image/enum.ImageFormat.html) that is **not** animated can be decoded.
        ///
        /// Stored as `Vec<u8>` for async-friendly transport, but typically consumed as a `&[u8]` during decoding.
        #[serde(with = "serde_bytes")]
        bytes: Vec<u8>,

        /// Horizontal offset from the left edge. We are using the top-left corner as the origin as seen in CSSOM.
        /// See https://developer.mozilla.org/en-US/docs/Web/API/CSSOM_view_API/Coordinate_systems
        ///
        /// The offset can be negative, so that image may appear cropped out of the `Frame`
        offset_left: i32,

        /// Vertical offset from the top edge. We are using the top-left corner as the origin as seen in CSSOM.
        /// See https://developer.mozilla.org/en-US/docs/Web/API/CSSOM_view_API/Coordinate_systems
        ///
        /// The offset can be negative, so that image may appear cropped out of the `Frame`
        offset_top: i32,

        /// Z-order for composition (0 = back, high = front).
        z_index: u32,
    },

    AnimatedImage {
        /// Raw encoded image data (GIF / APNG).
        /// This data is decoded via the `image` crate, thus any variant shown [here](https://docs.rs/image/latest/image/enum.ImageFormat.html) that is animated can be decoded.
        ///
        /// Stored as `Vec<u8>` for async-friendly transport, but typically consumed as a `&[u8]` during decoding.
        #[serde(with = "serde_bytes")]
        bytes: Vec<u8>,

        /// Horizontal offset from the left edge. We are using the top-left corner as the origin as seen in CSSOM.
        /// See https://developer.mozilla.org/en-US/docs/Web/API/CSSOM_view_API/Coordinate_systems
        ///
        /// The offset can be negative, so that image may appear cropped out of the `Frame`
        offset_left: i32,

        /// Vertical offset from the top edge. We are using the top-left corner as the origin as seen in CSSOM.
        /// See https://developer.mozilla.org/en-US/docs/Web/API/CSSOM_view_API/Coordinate_systems
        ///
        /// The offset can be negative, so that image may appear cropped out of the `Frame`
        offset_top: i32,

        /// Z-order for composition (0 = back, high = front).
        z_index: u32,
    },
}

/// Global display parameters applied to a batch of overlays.
#[derive(Serialize, Deserialize, Debug)]
pub struct DisplayOptions {
    /// Duration in milliseconds the overlay batch should remain visible.
    pub timeout_ms: u32,
}

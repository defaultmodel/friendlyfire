#[derive(Clone)]
pub struct Frame {
    /// Horizontal offset from the left edge. We are using the top-left corner as the origin as seen in CSSOM.
    /// See https://developer.mozilla.org/en-US/docs/Web/API/CSSOM_view_API/Coordinate_systems
    ///
    /// The offset can be negative
    pub left: i32,

    /// Vertical offset from the top edge. We are using the top-left corner as the origin as seen in CSSOM.
    /// See https://developer.mozilla.org/en-US/docs/Web/API/CSSOM_view_API/Coordinate_systems
    ///
    /// The offset can be negative
    pub top: i32,

    pub width: u32,
    pub height: u32,

    /// straight RGBA8.
    ///
    /// Conversion to other formats happens in the `Rendering` structs/traits specific to any `Window`
    /// Win32 : BGRA + premultiply
    /// X11 : ARGB
    /// Wayland : wl_shm RGBA or dmabuf upload
    pub buffer: Vec<u8>,

    /// Frame duration in milliseconds.
    /// 0 = treat as "static".
    // u128 was chosen for consistency reasons with the `image` crate
    pub delay_ms: u128,
}

impl Frame {
    pub fn new(left: i32, top: i32, width: u32, height: u32, delay_ms: u128) -> Self {
        Self {
            left,
            top,
            width,
            height,
            buffer: vec![0; (width * height * 4) as usize],
            delay_ms,
        }
    }

    /// Create a frame from RGBA bytes
    pub fn from_bytes(
        left: i32,
        top: i32,
        width: u32,
        height: u32,
        pixels: &[u8],
        delay_ms: u128,
    ) -> Self {
        assert_eq!(pixels.len(), (width * height * 4) as usize);
        Self {
            left,
            top,
            width,
            height,
            buffer: pixels.to_vec(),
            delay_ms,
        }
    }

    pub fn clear(&mut self) {
        self.buffer.fill(0);
    }

    /// Copy an RGBA image onto the frame buffer with no resizing.
    /// `src_pixels` must be exactly (src_width * src_height * 4) bytes.
    // TODO: This really should be GPU-accelerated
    pub fn blit(
        &mut self,
        dst_x: u32,
        dst_y: u32,
        src_width: u32,
        src_height: u32,
        src_pixels: &[u8],
    ) {
        assert_eq!(src_pixels.len(), (src_width * src_height * 4) as usize);

        let frame_width = self.width as usize;

        for row in 0..src_height as usize {
            for col in 0..src_width as usize {
                let dst_idx = (((dst_y as usize + row) * frame_width) + (dst_x as usize + col)) * 4;

                let src_idx = (row * src_width as usize + col) * 4;

                let src_r = src_pixels[src_idx] as f32;
                let src_g = src_pixels[src_idx + 1] as f32;
                let src_b = src_pixels[src_idx + 2] as f32;
                let src_a = src_pixels[src_idx + 3] as f32 / 255.0;

                let dst_r = self.buffer[dst_idx] as f32;
                let dst_g = self.buffer[dst_idx + 1] as f32;
                let dst_b = self.buffer[dst_idx + 2] as f32;
                let dst_a = self.buffer[dst_idx + 3] as f32 / 255.0;

                // https://wikimedia.org/api/rest_v1/media/math/render/svg/5c24c56475a4c3d86f6903f16195b866185f0551
                let out_a = src_a + dst_a * (1.0 - src_a);
                let out_r = (src_r * src_a + dst_r * dst_a * (1.0 - src_a)) / out_a;
                let out_g = (src_g * src_a + dst_g * dst_a * (1.0 - src_a)) / out_a;
                let out_b = (src_b * src_a + dst_b * dst_a * (1.0 - src_a)) / out_a;

                self.buffer[dst_idx] = out_r as u8;
                self.buffer[dst_idx + 1] = out_g as u8;
                self.buffer[dst_idx + 2] = out_b as u8;
                self.buffer[dst_idx + 3] = (out_a * 255.0) as u8;
            }
        }
    }
}

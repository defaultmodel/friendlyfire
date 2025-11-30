#[derive(Clone)]
pub struct Frame {
    pub left: i32,
    pub top: i32,

    pub width: u32,
    pub height: u32,

    /// straight or premultiplied RGBA8.
    /// Most backends will convert it to some other format
    /// Win32 : BGRA + premultiply
    /// X11 : ARGB
    /// Wayland : wl_shm RGBA or dmabuf upload
    pub buffer: Vec<u8>,

    /// Frame duration in milliseconds.
    /// 0 = treat as “static”.
    pub delay_ms: u32,
}

impl Frame {
    pub fn new(left: i32, top: i32, width: u32, height: u32, delay_ms: u32) -> Self {
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
        delay_ms: u32,
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

    /// Copy an RGBA image into the frame buffer with no resizing.
    /// `src_pixels` must be exactly (src_width * src_height * 4) bytes.
    pub fn blit(
        &mut self,
        dst_x: u32,
        dst_y: u32,
        src_width: u32,
        src_height: u32,
        src_pixels: &[u8],
    ) {
        let frame_width = self.width as usize;

        for row in 0..src_height as usize {
            let dst_offset = ((dst_y as usize + row) * frame_width + dst_x as usize) * 4;
            let src_offset = row * src_width as usize * 4;

            let dst_slice = &mut self.buffer[dst_offset..dst_offset + src_width as usize * 4];
            let src_slice = &src_pixels[src_offset..src_offset + src_width as usize * 4];

            dst_slice.copy_from_slice(src_slice);
        }
    }
}

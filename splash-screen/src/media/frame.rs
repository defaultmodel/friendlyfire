#[derive(Clone)]
pub struct Frame {
    pub width: u32,
    pub height: u32,

    /// straight or premultiplied RGBA8.
    /// Most backends will convert it to some other format
    /// Win32 : BGRA + premultiply
    /// X11 : ARGB
    /// Wayland : wl_shm RGBA or dmabuf upload
    pub rgba: Vec<u8>,

    /// Frame duration in milliseconds.
    /// 0 = treat as “static”.
    pub delay_ms: u32,

    /// Presentation timestamp (for video and animated images)
    /// > When should this frame be shown ?
    ///
    pub timestamp: Option<u64>,
}

impl Frame {
    pub fn new(width: u32, height: u32, rgba: Vec<u8>, delay_ms: u32) -> Self {
        Self {
            width,
            height,
            rgba,
            delay_ms,
            timestamp: None,
        }
    }

    pub fn is_static(&self) -> bool {
        self.delay_ms <= 1 && self.timestamp.is_none()
    }
}

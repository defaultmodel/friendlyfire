use cosmic_text::{Attrs, Buffer, Color, FontSystem, Metrics, Shaping, SwashCache};

use crate::{frame::Frame, overlay::Overlay};

/// Static text overlay rasterized as a bitmap.
pub struct TextOverlay {
    pub z_index: u32,
    pub frame: Frame,
}

impl TextOverlay {
    /// Create a text overlay by shaping and rasterizing text into a frame.
    pub fn from_bytes(
        font_manager: &mut FontSystem,
        mut swash_cache: &mut SwashCache,
        text: &str,
        font_size: u32,
        color: &[u8; 4],
        left: i32,
        top: i32,
        z_index: u32,
    ) -> anyhow::Result<Self> {
        // TODO : change this for a better heuristic. See https://grtcalculator.com/math/
        let line_height = font_size as f32 * 1.2;
        let text_color = Color::rgba(color[0], color[1], color[2], color[3]);
        let metrics = Metrics::new(font_size as f32, line_height);

        // Prepare the shaping buffer
        let mut buffer = Buffer::new(font_manager, metrics);
        let mut buf = buffer.borrow_with(font_manager);

        buf.set_size(None, None);

        let attrs = Attrs::new()
            .color(text_color)
            // TODO : Make it customizable, there must be a way to give arbitrary font_data !
            .family(cosmic_text::Family::Name("Algerian"));

        buf.set_text(text, &attrs, Shaping::Advanced, None);
        buf.shape_until_scroll(true);

        let line_count = buf.layout_runs().count() as u32;
        let height = (line_height as u32) * line_count;
        // TODO : Remove hardcoded value, but I'm not sure how just yet...
        let width = 1920;

        let mut pixels = vec![0; width * height as usize * 4];

        buf.draw(swash_cache, text_color, |x, y, w, h, color| {
            if color.a() == 0 || w != 1 || h != 1 {
                return;
            }

            if x < 0 || x >= width as i32 || y < 0 || y >= height as i32 {
                return;
            }

            let i = ((y as usize) * width + (x as usize)) * 4;
            pixels[i] = color.r();
            pixels[i + 1] = color.g();
            pixels[i + 2] = color.b();
            pixels[i + 3] = color.a();
        });

        let frame = Frame {
            left,
            top,
            width: width as u32,
            height,
            buffer: pixels,
            delay_ms: 0,
        };

        Ok(Self { z_index, frame })
    }
}

impl Overlay for TextOverlay {
    fn z_index(&self) -> u32 {
        self.z_index
    }

    fn draw(&self, target: &mut Frame, _timestamp_ms: u128) {
        target.blit(
            self.frame.left as u32,
            self.frame.top as u32,
            self.frame.width,
            self.frame.height,
            &self.frame.buffer,
        );
    }

    fn time_to_next_frame_ms(&self, _timestamp_ms: u128) -> Option<u128> {
        None
    }
}

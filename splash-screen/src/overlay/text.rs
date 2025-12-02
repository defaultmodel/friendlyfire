use cosmic_text::{Attrs, Buffer, Color, FontSystem, Metrics, Shaping, SwashCache};

use crate::{frame::Frame, overlay::Overlay};

pub struct TextOverlay {
    pub z_index: i32,
    pub frame: Frame,
}

impl TextOverlay {
    pub fn from_bytes(
        font_manager: &mut FontSystem,
        mut swash_cache: &mut SwashCache,
        font_data: &[u8],
        text: &str,
        font_size: u32,
        color: &[u8; 4],
        left: i32,
        top: i32,
        z_index: i32,
    ) -> anyhow::Result<Self> {
        let line_height: f32 = font_size as f32 * 1.2;

        let text_color = Color::rgba(color[0], color[1], color[2], color[3]);
        let metrics = Metrics::new(font_size as f32, line_height);

        let mut buffer = Buffer::new(font_manager, metrics);
        let mut buffer = buffer.borrow_with(font_manager);

        buffer.set_size(None, None);
        let attrs = Attrs::new()
            .color(text_color)
            // TODO Change debug
            .family(cosmic_text::Family::Name("Algerian"));
        buffer.set_text(text, &attrs, Shaping::Advanced, None);

        buffer.shape_until_scroll(true);

        let lines = buffer.layout_runs().count() as u32;

        let height = line_height as u32 * lines;
        let width = 1920;
        let mut canvas = vec![0; width * height as usize * 4];

        // Draw the buffer (for performance, instead use SwashCache directly)
        buffer.draw(swash_cache, text_color, |x, y, w, h, color| {
            let a = color.a();
            if a == 0
                || x < 0
                || x >= width as i32
                || y < 0
                || y >= height as i32
                || w != 1
                || h != 1
            {
                // Ignore alphas of 0, or invalid x, y coordinates, or unimplemented sizes
                return;
            }

            let idx = ((y as usize) * width + (x as usize)) * 4;

            canvas[idx] = color.r();
            canvas[idx + 1] = color.g();
            canvas[idx + 2] = color.b();
            canvas[idx + 3] = color.a();
        });

        // Build final frame
        let frame = Frame {
            left,
            top,
            width: width as u32,
            height,
            buffer: canvas,
            delay_ms: 0,
        };

        Ok(Self { z_index, frame })
    }
}

impl Overlay for TextOverlay {
    fn z_index(&self) -> i32 {
        self.z_index
    }

    fn draw(&self, target: &mut Frame, _timestamp_ms: u64) {
        target.blit(
            self.frame.left as u32,
            self.frame.top as u32,
            self.frame.width,
            self.frame.height,
            &self.frame.buffer,
        );
    }

    fn time_to_next_frame_ms(&self, _timestamp_ms: u64) -> Option<u64> {
        None
    }
}

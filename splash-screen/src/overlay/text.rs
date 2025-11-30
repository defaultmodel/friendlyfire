use fontdue::layout::{Layout, TextStyle};

use crate::{frame::Frame, overlay::Overlay};

pub struct TextOverlay {
    pub z_index: i32,
    pub frame: Frame,
}

impl TextOverlay {
    pub fn from_bytes(
        font_data: &[u8],
        text: &str,
        font_size: u32,
        color: &[u8; 4],
        left: i32,
        top: i32,
        z_index: i32,
    ) -> anyhow::Result<Self> {
        let font = fontdue::Font::from_bytes(font_data, fontdue::FontSettings::default()).unwrap();
        let fonts = &[font];

        let mut layout = Layout::new(fontdue::layout::CoordinateSystem::PositiveYDown);
        let style = TextStyle::new(text, font_size as f32, 0);
        layout.append(fonts, &style);

        let glyphs = layout.glyphs();

        // Compute bounding box
        let mut min_x = f32::MAX;
        let mut min_y = f32::MAX;
        let mut max_x = f32::MIN;
        let mut max_y = f32::MIN;

        for glyph in glyphs {
            min_x = min_x.min(glyph.x);
            min_y = min_y.min(glyph.y);

            max_x = max_x.max(glyph.x + glyph.width as f32);
            max_y = max_y.max(glyph.y + glyph.height as f32);
        }

        let width = (max_x - min_x).ceil() as u32;
        let height = (max_y - min_y).ceil() as u32;

        let mut buffer = vec![0u8; (width * height * 4) as usize];

        for glyph in glyphs {
            let (metrics, bitmap) = fonts[glyph.font_index]
                .rasterize_indexed_subpixel(glyph.key.glyph_index, font_size as f32);

            let bmp_width = metrics.width * 3; // rasterize_indexed_subpixel, renders at three time the size
            let gx = (glyph.x - min_x) as i32;
            let gy = (glyph.y - min_y) as i32;

            for y in 0..metrics.height {
                for x in 0..metrics.width {
                    let bx = x * 3;
                    let r = bitmap[y * bmp_width + bx];
                    let g = bitmap[y * bmp_width + bx + 1];
                    let b = bitmap[y * bmp_width + bx + 2];

                    if r == 0 && g == 0 && b == 0 {
                        continue;
                    }

                    let tx = gx + x as i32;
                    let ty = gy + y as i32;

                    if tx < 0 || ty < 0 || tx >= width as i32 || ty >= height as i32 {
                        continue;
                    }

                    let idx = ((ty as u32 * width + tx as u32) * 4) as usize;

                    buffer[idx] = (r as u32 * color[0] as u32 / 255) as u8;
                    buffer[idx + 1] = (g as u32 * color[1] as u32 / 255) as u8;
                    buffer[idx + 2] = (b as u32 * color[2] as u32 / 255) as u8;
                    buffer[idx + 3] = 255;
                }
            }
        }

        // Build final frame
        let frame = Frame {
            left,
            top,
            width,
            height,
            buffer,
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

use crate::frame::Frame;
use crate::media::decoded::DecodedMedia;
use crate::media::decoder::MediaFormatDecoder;
use image::ImageReader;
use std::io::Cursor;

pub struct BasicDecoder;

impl MediaFormatDecoder for BasicDecoder {
    fn decode(bytes: &[u8]) -> Option<DecodedMedia> {
        let img = ImageReader::new(Cursor::new(bytes))
            .with_guessed_format()
            .ok()?
            .decode()
            .ok()?
            .to_rgba8();

        let (w, h) = img.dimensions();
        Some(DecodedMedia::Static(Frame::new(w, h, img.into_raw(), 0)))
    }
}

use image::AnimationDecoder;

use crate::frame::Frame;
use crate::media::decoded::DecodedMedia;
use crate::media::decoder::MediaFormatDecoder;
use std::io::Cursor;

pub struct GifDecoder;

impl MediaFormatDecoder for GifDecoder {
    fn decode(bytes: &[u8]) -> Option<DecodedMedia> {
        let cursor = Cursor::new(bytes);
        let decoder = image::codecs::gif::GifDecoder::new(cursor).ok()?;
        let frames_iter = decoder.into_frames();
        let frames_vec = frames_iter.collect_frames().ok()?;

        let frames = frames_vec
            .into_iter()
            .map(|frame| {
                let (numerator, denominator) = frame.delay().numer_denom_ms();
                let delay_ms = numerator / denominator;
                let buffer = frame.into_buffer();
                Frame::new(buffer.width(), buffer.height(), buffer.into_vec(), delay_ms)
            })
            .collect();

        Some(DecodedMedia::Animated(frames))
    }
}

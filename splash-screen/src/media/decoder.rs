use crate::media::image::{BasicDecoder, GifDecoder};

use super::decoded::DecodedMedia;

pub trait MediaFormatDecoder {
    /// Attempt to decode bytes into a media object.
    /// Return None if this decoder cannot handle the data.
    fn decode(bytes: &[u8]) -> Option<DecodedMedia>;
}

pub struct MediaDecoder {}

impl MediaDecoder {
    /// Try to decode raw bytes into a DecodedMedia
    pub fn decode(bytes: &[u8]) -> Option<DecodedMedia> {
        match sniff_format(bytes) {
            MediaFormat::Png | MediaFormat::Jpeg | MediaFormat::Webp => BasicDecoder::decode(bytes),
            MediaFormat::Gif => GifDecoder::decode(bytes),
            MediaFormat::Mp4 => todo!(),
            MediaFormat::Unknown => BasicDecoder::decode(bytes),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum MediaFormat {
    Png,
    Gif,
    Jpeg,
    Webp,
    Mp4,
    Unknown,
}

fn sniff_format(bytes: &[u8]) -> MediaFormat {
    if bytes.starts_with(b"\x89PNG\r\n\x1A\n") {
        return MediaFormat::Png;
    }
    if bytes.starts_with(&[0xFF, 0xD8, 0xFF]) {
        return MediaFormat::Jpeg;
    }
    if bytes.starts_with(b"GIF87a") || bytes.starts_with(b"GIF89a") {
        return MediaFormat::Gif;
    }
    if bytes.len() > 8 && &bytes[4..8] == b"ftyp" {
        return MediaFormat::Mp4;
    }

    MediaFormat::Unknown
}

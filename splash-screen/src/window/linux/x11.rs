use crate::media::decoded::DecodedMedia;
use crate::window::traits::SplashWindow;

pub struct X11Window {}

impl SplashWindow for X11Window {
    fn new() -> Self {
        todo!()
    }

    fn show(&self) {
        todo!()
    }
    fn hide(&self) {
        todo!()
    }
    fn destroy(&self) {
        todo!()
    }
    fn resize(&self, w: u32, h: u32) {
        todo!()
    }
    fn show_media(&self, media: DecodedMedia) {
        todo!()
    }
    fn clear(&self) {
        todo!()
    }
}

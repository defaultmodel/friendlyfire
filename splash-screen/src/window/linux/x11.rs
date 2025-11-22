use friendlyfire_shared_lib::DisplayOptions;

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
    fn show_media(&self, media: DecodedMedia, options: DisplayOptions) {
        todo!()
    }
    fn clear(&self) {
        todo!()
    }
}

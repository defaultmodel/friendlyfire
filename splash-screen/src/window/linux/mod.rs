mod wayland;
mod x11;

use std::env;

use friendlyfire_shared_lib::DisplayOptions;

use crate::{media::decoded::DecodedMedia, window::traits::SplashWindow};

pub enum LinuxSplashWindow {
    Wayland(wayland::WaylandWindow),
    X11(x11::X11Window),
}

/// Dispatcher struct: implements SplashWindow and delegates to backend
impl LinuxSplashWindow {
    pub fn new() -> Self {
        match env::var("WAYLAND_DISPLAY") {
            Ok(_) => LinuxSplashWindow::Wayland(wayland::WaylandWindow::new()),
            Err(_) => LinuxSplashWindow::X11(x11::X11Window::new()),
        }
    }

    pub fn show(&self) {
        match self {
            LinuxSplashWindow::Wayland(w) => w.show(),
            LinuxSplashWindow::X11(w) => w.show(),
        }
    }
    pub fn hide(&self) {
        match self {
            LinuxSplashWindow::Wayland(w) => w.hide(),
            LinuxSplashWindow::X11(w) => w.hide(),
        }
    }
    pub fn destroy(&self) {
        match self {
            LinuxSplashWindow::Wayland(w) => w.destroy(),
            LinuxSplashWindow::X11(w) => w.destroy(),
        }
    }
    pub async fn show_media(&self, media: DecodedMedia, options: DisplayOptions) {
        match self {
            LinuxSplashWindow::Wayland(w) => w.show_media(media, options).await,
            LinuxSplashWindow::X11(w) => w.show_media(media, options).await,
        }
    }
    pub fn clear(&self) {
        match self {
            LinuxSplashWindow::Wayland(w) => w.clear(),
            LinuxSplashWindow::X11(w) => w.clear(),
        }
    }
}

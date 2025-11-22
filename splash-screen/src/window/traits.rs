use friendlyfire_shared_lib::DisplayOptions;

use crate::media::decoded::DecodedMedia;

pub trait SplashWindow {
    /// Create the platform window (transparent, borderless, topmost)
    fn new() -> Self;

    /// Show the window (no-op if already visible)
    fn show(&self);

    /// Hide the window, but keep resources alive
    fn hide(&self);

    /// Completely destroy the window
    fn destroy(&self);

    /// Render a full decoded media from start to end
    fn show_media(&self, media: DecodedMedia, options: DisplayOptions);

    /// Clear the window to transparency
    fn clear(&self);
}

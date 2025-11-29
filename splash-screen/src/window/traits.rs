pub trait SplashWindow {
    /// Create a transparent, borderless, always-on-top window.
    fn create() -> anyhow::Result<Self>
    where
        Self: Sized;

    /// Show the window (no-op if already visible)
    fn show(&mut self);

    /// Hide the window but keep resources alive
    fn hide(&mut self);

    /// Release all native resources
    fn destroy(&mut self);

    /// Clear the window to full transparency
    fn clear(&mut self);
}

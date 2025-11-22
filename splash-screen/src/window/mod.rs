mod linux;
mod traits;
mod win32;

pub use traits::SplashWindow;

#[cfg(target_os = "windows")]
pub use win32::Win32Window as PlatformSplashWindow;

#[cfg(target_os = "linux")]
pub use linux::LinuxSplashWindow as PlatformSplashWindow;

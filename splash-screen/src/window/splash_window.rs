use std::sync::Mutex;
use windows::{
    Win32::{Foundation::*, Graphics::Direct2D::*, UI::WindowsAndMessaging::*},
    core::*,
};

use super::{rendering, winapi};

pub trait SplashWindow {
    fn new() -> Self;
    fn show(&self);
    fn hide(&self);
    fn update_image(&self, bytes: &[u8]);
    fn clear(&self);
}

pub struct Window {
    pub handle: HWND,
    pub direct2d_factory: ID2D1Factory,
    pub render_target: Mutex<Option<ID2D1HwndRenderTarget>>,
    pub bitmap: Mutex<Option<ID2D1Bitmap>>,
}

impl SplashWindow for Window {
    /// Register a class name and creates the window
    /// NOTE: The windows is not visible at this point ! You need to call show()
    fn new() -> Self {
        let class_name = s!("friendlyfire-splash-screen");
        let instance = unsafe { winapi::register_window_class(class_name).unwrap() };
        let window_handle = unsafe { winapi::create_window(class_name, instance).unwrap() };

        let direct2d_factory = rendering::create_factory();

        Self {
            handle: window_handle,
            direct2d_factory,
            render_target: Mutex::new(None),
            bitmap: Mutex::new(None),
        }
    }

    fn show(&self) {
        unsafe {
            ShowWindow(self.handle, SW_SHOW).unwrap();
        };
    }

    fn hide(&self) {
        unsafe {
            ShowWindow(self.handle, SW_HIDE).unwrap();
        };
    }

    /// Updates the windows with a new image
    /// NOTE: This does not show the window, use show() for this
    fn update_image(&self, bytes: &[u8]) {
        rendering::update_image(self, bytes);
    }

    /// Clear the windows and makes it dissappear from view
    fn clear(&self) {
        todo!()
    }
}

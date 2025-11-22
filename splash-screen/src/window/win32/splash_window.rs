use std::time::Duration;

use windows::Win32::{Foundation::*, Graphics::Gdi::*, UI::WindowsAndMessaging::*};
use windows::core::*;

use crate::{
    media::{decoded::DecodedMedia, frame::Frame},
    window::{
        traits::SplashWindow,
        win32::{rendering, winapi},
    },
};

pub struct Win32Window {
    pub handle: HWND,
}

impl SplashWindow for Win32Window {
    /// Register a class name and creates the window
    /// NOTE: The windows is not visible at this point ! You need to call show()
    fn new() -> Self {
        let class_name = s!("friendlyfire-splash-screen");
        let instance = unsafe { winapi::register_window_class(class_name).unwrap() };
        let window_handle = unsafe { winapi::create_layered_window(class_name, instance).unwrap() };

        Self {
            handle: window_handle,
        }
    }

    fn show(&self) {
        unsafe { ShowWindow(self.handle, SW_SHOW).unwrap() };
    }

    fn hide(&self) {
        unsafe { ShowWindow(self.handle, SW_HIDE).unwrap() };
    }

    fn destroy(&self) {
        unsafe { DestroyWindow(self.handle).unwrap() }
    }

    fn resize(&self, width: u32, height: u32) {
        todo!()
    }

    fn show_media(&self, media: DecodedMedia) {
        match media {
            DecodedMedia::Static(frame) => self.draw_frame(&frame),
            DecodedMedia::Animated(frames) => {
                for frame in frames.iter() {
                    println!("drawing new frame ");
                    self.draw_frame(frame);
                    std::thread::sleep(Duration::from_millis(frame.delay_ms.max(1) as u64));
                }
            }
            DecodedMedia::Video(video_stream) => todo!(),
        }
    }
    /// Clear the windows and makes it dissappear from view
    fn clear(&self) {
        todo!()
    }
    // Updates the windows with a new image
    // NOTE: This does not show the window, use show() for this
    // fn update_image(&self, bytes: &[u8]) {
    //     rendering::update_image(self, bytes);
    // }
}

impl Win32Window {
    /// Render a single decoded frame (static, animation frame or video frame)
    fn draw_frame(&self, frame: &Frame) {
        let bgra = rendering::rgba_to_premultiplied_bgra(&frame.rgba);

        unsafe {
            // create compatible DC
            let hdc_screen = GetDC(HWND(0));
            let mem_dc = CreateCompatibleDC(hdc_screen);
            ReleaseDC(HWND(0), hdc_screen);

            // create DIB section and copy pixels
            let dib = rendering::create_dib_section(mem_dc, frame.width, frame.height, &bgra);
            let old = SelectObject(mem_dc, dib);

            // update layered window
            rendering::update_layered(self.handle, mem_dc, frame.width, frame.height);

            // cleanup
            SelectObject(mem_dc, old);
            if let BOOL(0) = DeleteObject(dib) {
                eprintln!("Unable to delete DIB object");
                panic!();
            }
            if let BOOL(0) = DeleteDC(mem_dc) {
                eprintln!("Unable to delete DC");
                panic!();
            }
        }
    }
}

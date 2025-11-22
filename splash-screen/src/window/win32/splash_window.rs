use std::time::Duration;

use friendlyfire_shared_lib::DisplayOptions;
use tokio::time;
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

    async fn show_media(&self, media: DecodedMedia, options: DisplayOptions) {
        let timeout = time::Duration::from_millis(options.timeout_ms as u64);
        let start = time::Instant::now();
        match media {
            DecodedMedia::Static(frame) => {
                self.draw_frame(&frame);
                time::sleep(timeout).await;
                self.clear();
            }
            DecodedMedia::Animated(frames) => {
                let mut idx = 0;

                loop {
                    if time::Instant::now().duration_since(start) >= timeout {
                        break;
                    }

                    let frame = &frames[idx];
                    let delay = frame.delay_ms.max(1) as u64;
                    let frame_duration = time::Duration::from_millis(delay);

                    let draw_start = time::Instant::now();
                    self.draw_frame(frame);
                    let draw_elapsed = draw_start.elapsed();

                    if draw_elapsed < frame_duration {
                        time::sleep(frame_duration - draw_elapsed).await;
                    }

                    idx = (idx + 1) % frames.len();
                }

                self.clear();
            }
            DecodedMedia::Video(video_stream) => todo!(),
        }
    }
    /// Clear the windows and makes it dissappear from view
    fn clear(&self) {
        rendering::clear(self.handle)
    }
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

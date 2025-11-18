use std::fs;

use windows::{
    Win32::{
        System::Com::*,
        UI::WindowsAndMessaging::{DispatchMessageA, TranslateMessage},
    },
    core::*,
};

use crate::window::SplashWindow;

mod window;

struct App {}

impl App {
    pub unsafe fn run() -> Result<()> {
        unsafe {
            Self::initialize()?;
        }
        let window = window::Window::new();
        window.show();

        let image_bytes = fs::read("bonk.png")?;
        window.update_image(&image_bytes);

        // Basic Win32 message loop
        unsafe {
            let mut msg = std::mem::zeroed();
            while windows::Win32::UI::WindowsAndMessaging::GetMessageA(
                &mut msg,
                windows::Win32::Foundation::HWND(0),
                0,
                0,
            )
            .into()
            {
                let _ = TranslateMessage(&msg);
                DispatchMessageA(&msg);
            }
        }

        unsafe {
            Self::uninitialize()?;
        }
        Ok(())
    }

    unsafe fn initialize() -> Result<()> {
        unsafe { CoInitializeEx(None, COINIT_APARTMENTTHREADED).unwrap() };
        Ok(())
    }

    unsafe fn uninitialize() -> Result<()> {
        unsafe { CoUninitialize() };
        Ok(())
    }
}

fn main() -> Result<()> {
    unsafe { App::run() }
}

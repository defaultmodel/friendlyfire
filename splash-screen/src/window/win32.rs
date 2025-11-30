use windows::{
    Win32::{
        Foundation::*, Graphics::Gdi::Rectangle, System::LibraryLoader::*,
        UI::WindowsAndMessaging::*,
    },
    core::*,
};

use crate::window::traits::SplashWindow;

pub struct Win32Window {
    pub handle: HWND,
    pub thread: std::thread::JoinHandle<()>,
}

impl SplashWindow for Win32Window {
    fn create() -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        let (tx, rx) = std::sync::mpsc::sync_channel(1);

        let thread = std::thread::spawn(move || {
            unsafe {
                let classname = s!("friendlyfire-splash-screen");
                let instance = register_window_class(classname).unwrap();
                let handle = create_layered_window(classname, instance).unwrap();

                // Send handle back
                tx.send(handle).unwrap();

                // Message loop
                let mut msg = std::mem::zeroed();
                while GetMessageA(&mut msg, HWND(0), 0, 0).into() {
                    let _ = TranslateMessage(&msg);
                    DispatchMessageA(&msg);
                }
            }
        });

        let handle = rx.recv()?; // Wait for the thread to create the window
        Ok(Self { handle, thread })
    }

    fn show(&mut self) {
        unsafe { ShowWindow(self.handle, SW_SHOW).unwrap() };
    }

    fn hide(&mut self) {
        unsafe { ShowWindow(self.handle, SW_HIDE).unwrap() };
    }

    fn destroy(&mut self) {
        unsafe { DestroyWindow(self.handle).unwrap() }
    }

    fn clear(&mut self) {
        todo!()
    }

    fn dimensions(&self) -> (u32, u32) {
        let mut dimensions = RECT::default();
        unsafe {
            // fills the dimensions struct
            GetWindowRect(self.handle, &mut dimensions).unwrap();
        }
        // https://learn.microsoft.com/en-us/windows/win32/gdi/window-coordinate-system
        // Points on the screen are described by x and y coordinate pairs.
        // The x-coordinates increase to the right; y-coordinates increase from top to bottom
        let width = dimensions.right as u32 - dimensions.left as u32;
        let height = dimensions.bottom as u32 - dimensions.top as u32;
        (width, height)
    }
}

pub unsafe fn register_window_class(classname: PCSTR) -> Result<HINSTANCE> {
    // An HMODULE is the same thing as an instance
    // This is why I .into() it
    let instance: HINSTANCE = unsafe { GetModuleHandleA(None)?.into() };
    debug_assert!(instance.0 != 0);
    let window_class = WNDCLASSA {
        hInstance: instance,
        lpszClassName: classname,
        hCursor: unsafe { LoadCursorW(None, IDC_ARROW) }?,
        lpfnWndProc: Some(wndproc),
        ..Default::default()
    };
    let atom = unsafe { RegisterClassA(&window_class) };
    debug_assert!(atom != 0);
    Ok(instance)
}

pub unsafe fn create_layered_window(classname: PCSTR, window_instance: HINSTANCE) -> Result<HWND> {
    unsafe {
        let window_extended_style = WS_EX_LAYERED |
            WS_EX_TOPMOST | // always-on-top
            WS_EX_NOACTIVATE | // click-through
            WS_EX_TOOLWINDOW; // remove taskbar icon
        let window_style = WS_POPUP | WS_VISIBLE;

        let screen_width = GetSystemMetrics(SM_CXSCREEN);
        let screen_height = GetSystemMetrics(SM_CYSCREEN);

        let window_handle = CreateWindowExA(
            window_extended_style,
            classname,
            classname,
            window_style,
            0,
            0,
            screen_width,
            screen_height,
            None,
            None,
            window_instance,
            None,
        );

        if window_handle.0 == 0 {
            let error = GetLastError();
            println!("CreateWindowEx failed : {:?}", error);
        }
        Ok(window_handle)
    }
}

extern "system" fn wndproc(window: HWND, message: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    unsafe {
        match message {
            WM_PAINT => {
                println!("WM_PAINT");
                LRESULT(0)
            }
            WM_DESTROY => {
                println!("WM_DESTROY");
                PostQuitMessage(0);
                LRESULT(0)
            }
            _ => DefWindowProcA(window, message, wparam, lparam),
        }
    }
}

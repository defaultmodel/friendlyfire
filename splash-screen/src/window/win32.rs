use windows::{
    Win32::{Foundation::*, System::LibraryLoader::*, UI::WindowsAndMessaging::*},
    core::*,
};

use crate::window::traits::SplashWindow;

/// A Win32 window that implement the `SplashWindow` trait
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

        // RECT gives the coordinates of the window upper-left and lower-right corners.
        // https://learn.microsoft.com/en-us/windows/win32/api/windef/ns-windef-rect
        //
        // Coordinate pairs are used with a top-left origin.
        // https://learn.microsoft.com/en-us/windows/win32/gdi/window-coordinate-system
        let width = dimensions.right as u32 - dimensions.left as u32;
        let height = dimensions.bottom as u32 - dimensions.top as u32;

        (width, height)
    }
}

/// Each window is associated with a particular class, and once the class is registered with the system, windows of that class can be created.
/// See https://en.wikibooks.org/wiki/Windows_Programming/Window_Creation
pub unsafe fn register_window_class(classname: PCSTR) -> Result<HINSTANCE> {
    // In Win32, an `HMODULE` is equivalent to an `HINSTANCE`, explaining the `.into()`
    let instance: HINSTANCE = unsafe { GetModuleHandleA(None)?.into() };
    // TODO : Fix this shitty error handling, with `GetLastError`
    debug_assert!(instance.0 != 0);

    // Some options for the windows
    // TODO : Change the `hIcon` to the logo
    // TODO : Maybe change the cursor to something funny ??
    let window_class = WNDCLASSA {
        hInstance: instance,
        lpszClassName: classname,
        hCursor: unsafe { LoadCursorW(None, IDC_ARROW) }?,
        lpfnWndProc: Some(wndproc),
        ..Default::default()
    };

    // Register the window class with the system.
    // The returned ATOM uniquely identifies the class.
    // TODO : Switch to RegisterClassW as per recommandation : https://learn.microsoft.com/en-us/windows/win32/intl/registering-window-classes
    let atom = unsafe { RegisterClassA(&window_class) };
    // TODO : Fix this shitty error handling, with `GetLastError`
    debug_assert!(atom != 0);

    Ok(instance)
}

/// Create a fullscreen, transparent, always-on-top layered window.
///
/// The window uses the class previously registered via `register_window_class`
pub unsafe fn create_layered_window(classname: PCSTR, window_instance: HINSTANCE) -> Result<HWND> {
    unsafe {
        let window_extended_style = WS_EX_LAYERED |
            WS_EX_TOPMOST | // always-on-top
            WS_EX_NOACTIVATE | // not foreground when clicked on
            WS_EX_TRANSPARENT | // click-through
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

/// Procedure used by the window class
/// This handles messages received by the given window
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

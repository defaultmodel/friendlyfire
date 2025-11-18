use windows::{
    Win32::{Foundation::*, System::LibraryLoader::*, UI::WindowsAndMessaging::*},
    core::*,
};

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

pub unsafe fn create_window(classname: PCSTR, window_instance: HINSTANCE) -> Result<HWND> {
    unsafe {
        let window_extended_style = WS_EX_TOPMOST;
        let window_style = WS_OVERLAPPEDWINDOW | WS_VISIBLE;

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

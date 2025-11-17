use windows::{
    Win32::{
        Foundation::*,
        Graphics::{Direct2D::*, Gdi::*},
        System::{Com::*, LibraryLoader::*},
        UI::WindowsAndMessaging::*,
    },
    core::*,
};

unsafe fn init_direct2d() -> Result<ID2D1Factory> {
    let d2d1_options = D2D1_FACTORY_OPTIONS {
        debugLevel: D2D1_DEBUG_LEVEL_NONE,
    };
    let id2d1_factory: ID2D1Factory = unsafe {
        D2D1CreateFactory(D2D1_FACTORY_TYPE_MULTI_THREADED, Some(&d2d1_options)).unwrap()
    };
    Ok(id2d1_factory)
}

unsafe fn register_window_class(classname: PCSTR) -> Result<HINSTANCE> {
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

unsafe fn create_window(classname: PCSTR, window_instance: HINSTANCE) -> Result<()> {
    unsafe {
        let window_extended_style = WS_EX_TOPMOST;
        let window_style = WS_OVERLAPPEDWINDOW | WS_VISIBLE; // borderless

        let window_handle = CreateWindowExA(
            window_extended_style,
            classname,
            classname,
            window_style,
            100,
            100,
            800,
            600,
            None,
            None,
            window_instance,
            None,
        );

        if window_handle.0 == 0 {
            let error = GetLastError();
            println!("CreateWindowEx failed : {:?}", error);
        }
        Ok(())
    }
}

fn main() -> Result<()> {
    unsafe {
        // Init COM
        CoInitializeEx(None, COINIT_APARTMENTTHREADED).unwrap();

        let classname = s!("friendlyfire-splash-screen");
        let window_instance = register_window_class(classname).unwrap();
        create_window(classname, window_instance).unwrap();

        let mut message = MSG::default();

        while GetMessageA(&mut message, HWND(0), 0, 0).into() {
            DispatchMessageA(&message);
        }

        let factory = init_direct2d()?;

        CoUninitialize();
    }
    Ok(())
}

extern "system" fn wndproc(window: HWND, message: u32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    unsafe {
        match message {
            WM_PAINT => {
                println!("WM_PAINT");
                ValidateRect(window, None).unwrap();
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

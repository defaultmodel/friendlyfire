use std::{ffi::c_void, io::Cursor, sync::Mutex};

use image::ImageReader;
use windows::{
    Win32::{
        Foundation::*,
        Graphics::{
            Direct2D::{Common::*, *},
            Dxgi::Common::*,
            Gdi::*,
        },
        System::LibraryLoader::*,
        UI::WindowsAndMessaging::*,
    },
    core::*,
};

pub trait SplashWindow {
    fn new() -> Self;
    fn show(&self);
    fn hide(&self);
    fn update_image(&self, bytes: &[u8]);
    fn clear(&self);
}

pub struct Window {
    handle: HWND,
    direct2d_factory: ID2D1Factory,
    render_target: Mutex<Option<ID2D1HwndRenderTarget>>,
    bitmap: Mutex<Option<ID2D1Bitmap>>,
}

impl SplashWindow for Window {
    /// Register a class name and creates the window
    /// NOTE: The windows is not visible at this point ! You need to call show()
    fn new() -> Self {
        let class_name = s!("friendlyfire-splash-screen");
        let instance = unsafe { register_window_class(class_name).unwrap() };
        let window_handle = unsafe { create_window(class_name, instance).unwrap() };
        // unsafe { SetLayeredWindowAttributes(window_handle, COLORREF(0), 255, LWA_ALPHA).unwrap() }; // overall alpha

        let direct2d_factory: ID2D1Factory = unsafe {
            D2D1CreateFactory(
                D2D1_FACTORY_TYPE_MULTI_THREADED,
                Some(&D2D1_FACTORY_OPTIONS {
                    debugLevel: D2D1_DEBUG_LEVEL_NONE,
                }),
            )
            .unwrap()
        };

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
        // Decode PNG
        let image = ImageReader::new(Cursor::new(bytes))
            .with_guessed_format()
            .unwrap()
            .decode()
            .unwrap()
            .to_rgba8();
        let (width, height) = image.dimensions();
        let image_bytes = image.into_raw();

        // convert to premultiplied BGRA
        let mut bgra: Vec<u8> = Vec::with_capacity(image_bytes.len());
        // src layout: [R,G,B,A, R,G,B,A, ...]
        // target layout: [B_premul, G_premul, R_premul, A, ...]
        for chunk in image_bytes.chunks_exact(4) {
            let r = chunk[0] as u32;
            let g = chunk[1] as u32;
            let b = chunk[2] as u32;
            let a = chunk[3] as u32;
            if a == 0 {
                bgra.push(0);
                bgra.push(0);
                bgra.push(0);
                bgra.push(0);
            } else {
                // premultiply with rounding: (c * a + 127) / 255
                let r_p = ((r * a + 127) / 255) as u8;
                let g_p = ((g * a + 127) / 255) as u8;
                let b_p = ((b * a + 127) / 255) as u8;
                bgra.push(b_p);
                bgra.push(g_p);
                bgra.push(r_p);
                bgra.push(a as u8);
            }
        }

        // Initialize render target if needed
        let mut render_target_lock = self.render_target.lock().unwrap();
        if render_target_lock.is_none() {
            let render_target_props = D2D1_RENDER_TARGET_PROPERTIES {
                r#type: D2D1_RENDER_TARGET_TYPE_DEFAULT,
                pixelFormat: D2D1_PIXEL_FORMAT {
                    format: DXGI_FORMAT_B8G8R8A8_UNORM,
                    alphaMode: D2D1_ALPHA_MODE_PREMULTIPLIED,
                },
                dpiX: 0.0,
                dpiY: 0.0,
                usage: D2D1_RENDER_TARGET_USAGE_NONE,
                minLevel: D2D1_FEATURE_LEVEL_DEFAULT,
            };
            let hwnd_rt_props = D2D1_HWND_RENDER_TARGET_PROPERTIES {
                hwnd: self.handle,
                pixelSize: D2D_SIZE_U { width, height },
                presentOptions: D2D1_PRESENT_OPTIONS_NONE,
            };
            unsafe {
                let hwnd_rt = self
                    .direct2d_factory
                    .CreateHwndRenderTarget(&render_target_props, &hwnd_rt_props)
                    .expect("Failed to create Direct2D HwndRenderTarget");
                *render_target_lock = Some(hwnd_rt);
            };
        }

        let render_target = render_target_lock.as_ref().unwrap();

        // Create bitmap
        let mut bmp_lock = self.bitmap.lock().unwrap();
        let bmp_props = D2D1_BITMAP_PROPERTIES {
            pixelFormat: D2D1_PIXEL_FORMAT {
                format: DXGI_FORMAT_B8G8R8A8_UNORM,
                alphaMode: D2D1_ALPHA_MODE_PREMULTIPLIED,
            },
            dpiX: 96.0,
            dpiY: 96.0,
        };

        let size = D2D_SIZE_U { width, height };
        let bmp: ID2D1Bitmap = unsafe {
            render_target
                .CreateBitmap(
                    size,
                    Some(bgra.as_ptr() as *const c_void),
                    4 * width,
                    &bmp_props,
                )
                .unwrap()
        };
        *bmp_lock = Some(bmp.clone());

        // Draw
        unsafe {
            render_target.BeginDraw();
            render_target.Clear(Some(&D2D1_COLOR_F {
                r: 0.0,
                g: 0.0,
                b: 0.0,
                a: 0.0,
            }));
            render_target.DrawBitmap(&bmp, None, 1.0, D2D1_BITMAP_INTERPOLATION_MODE_LINEAR, None);
            render_target.EndDraw(None, None).unwrap();
        };
    }

    /// Clear the windows and makes it dissappear from view
    fn clear(&self) {
        todo!()
    }
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

unsafe fn create_window(classname: PCSTR, window_instance: HINSTANCE) -> Result<HWND> {
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

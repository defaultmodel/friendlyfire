use std::{ffi::c_void, mem::size_of, ptr};

use windows::Win32::{Foundation::*, Graphics::Gdi::*, UI::WindowsAndMessaging::*};

use crate::{frame::Frame, window::win32::Win32Window};

/// Abstracted renderer capable of presenting a `Frame` onto a `SplashWindow`.
pub trait Win32Renderer {
    /// Render a complete `Frame` to the layered window
    fn draw_frame(&self, frame: &Frame);
}

impl Win32Renderer for Win32Window {
    fn draw_frame(&self, frame: &Frame) {
        // windows uses pre-multiplied BGRA
        // https://stackoverflow.com/a/74925357
        let bgra = rgba_to_premultiplied_bgra(&frame.buffer);

        unsafe {
            let mem_dc = create_compatible_dc();
            let dib = create_dib_section(mem_dc, frame.width, frame.height, &bgra);
            let old_obj = SelectObject(mem_dc, dib);

            update_layered_window(self.handle, mem_dc, frame.width, frame.height);

            cleanup_dc(mem_dc, dib, old_obj);
        }
    }
}

/// Convert an RGBA buffer into premultiplied BGRA.
pub fn rgba_to_premultiplied_bgra(src: &[u8]) -> Vec<u8> {
    let mut bgra = Vec::with_capacity(src.len());

    for px in src.chunks_exact(4) {
        let (r, g, b, a) = (px[0] as u32, px[1] as u32, px[2] as u32, px[3] as u32);

        if a == 0 {
            bgra.extend_from_slice(&[0, 0, 0, 0]);
        } else {
            let r_p = ((r * a + 127) / 255) as u8;
            let g_p = ((g * a + 127) / 255) as u8;
            let b_p = ((b * a + 127) / 255) as u8;
            bgra.extend_from_slice(&[b_p, g_p, r_p, a as u8]);
        }
    }

    bgra
}

/// Create a memory device context compatible with the primary screen.
///
/// # Safety
/// The returned `HDC` must be freed with `DeleteDC`.
unsafe fn create_compatible_dc() -> HDC {
    unsafe {
        let screen_dc = GetDC(HWND(0));
        let mem_dc = CreateCompatibleDC(screen_dc);
        ReleaseDC(HWND(0), screen_dc);
        mem_dc
    }
}

/// Create a 32-bit top-down DIB section and copy pixel data into it.
///
/// # Safety
/// The returned `HBITMAP` must be freed with `DeleteObject`.
pub unsafe fn create_dib_section(mem_dc: HDC, width: u32, height: u32, bgra: &[u8]) -> HBITMAP {
    let header = BITMAPINFOHEADER {
        biSize: size_of::<BITMAPINFOHEADER>() as u32,
        biWidth: width as i32,
        biHeight: -(height as i32), // top-down
        biPlanes: 1,
        biBitCount: 32,
        biCompression: BI_RGB.0,
        ..Default::default()
    };

    let info = BITMAPINFO {
        bmiHeader: header,
        ..Default::default()
    };
    let mut out_ptr: *mut c_void = ptr::null_mut();

    let bitmap =
        unsafe { CreateDIBSection(mem_dc, &info, DIB_RGB_COLORS, &mut out_ptr, None, 0).unwrap() };
    unsafe { ptr::copy_nonoverlapping(bgra.as_ptr(), out_ptr as *mut u8, bgra.len()) };
    bitmap
}

/// Update a layered window with the contents of a memory DC.
///
/// Positions the window centered on the primary screen.
unsafe fn update_layered_window(win_handle: HWND, mem_dc: HDC, width: u32, height: u32) {
    let screen_dc = unsafe { GetDC(HWND(0)) };

    let size = SIZE {
        cx: width as i32,
        cy: height as i32,
    };
    let center = POINT {
        x: (unsafe { GetSystemMetrics(SM_CXSCREEN) } - width as i32) / 2,
        y: (unsafe { GetSystemMetrics(SM_CYSCREEN) } - height as i32) / 2,
    };
    let zero = POINT { x: 0, y: 0 };

    let blend = BLENDFUNCTION {
        BlendOp: AC_SRC_OVER as u8,
        BlendFlags: 0,
        SourceConstantAlpha: 255,
        AlphaFormat: AC_SRC_ALPHA as u8,
    };

    unsafe {
        UpdateLayeredWindow(
            win_handle,
            screen_dc,
            Some(&center),
            Some(&size),
            mem_dc,
            Some(&zero),
            COLORREF(0),
            Some(&blend),
            ULW_ALPHA,
        )
        .unwrap()
    };
    unsafe { ReleaseDC(HWND(0), screen_dc) };
}

/// Frees DC state and release GDI resources.
unsafe fn cleanup_dc(mem_dc: HDC, dib: HBITMAP, old_obj: HGDIOBJ) {
    unsafe {
        SelectObject(mem_dc, old_obj);
        if DeleteObject(dib) == BOOL(0) {
            panic!("Failed to delete DIB object");
        }
        if DeleteDC(mem_dc) == BOOL(0) {
            panic!("Failed to delete DC");
        }
    }
}

/// Clear a layered window by drawing a fully transparent pixel.
pub fn clear(hwnd: HWND) {
    unsafe {
        let mem_dc = create_compatible_dc();

        let pixel = [0u8; 4];
        let dib = create_dib_section(mem_dc, 1, 1, &pixel);
        let old_obj = SelectObject(mem_dc, dib);

        let blend = BLENDFUNCTION {
            BlendOp: AC_SRC_OVER as u8,
            BlendFlags: 0,
            SourceConstantAlpha: 255,
            AlphaFormat: AC_SRC_ALPHA as u8,
        };

        let pos = POINT { x: 0, y: 0 };
        let size = SIZE { cx: 1, cy: 1 };
        let src = POINT { x: 0, y: 0 };

        let screen_dc = GetDC(HWND(0));
        UpdateLayeredWindow(
            hwnd,
            screen_dc,
            Some(&pos),
            Some(&size),
            mem_dc,
            Some(&src),
            COLORREF(0),
            Some(&blend),
            ULW_ALPHA,
        )
        .unwrap();
        ReleaseDC(HWND(0), screen_dc);

        cleanup_dc(mem_dc, dib, old_obj);
    }
}

use std::{ffi::c_void, io::Cursor, mem::size_of, ptr};

use image::ImageReader;
use windows::Win32::{Foundation::*, Graphics::Gdi::*, UI::WindowsAndMessaging::*};

use crate::window::win32::Win32Window;

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

pub unsafe fn create_dib_section(mem_dc: HDC, width: u32, height: u32, bgra: &[u8]) -> HBITMAP {
    let header = BITMAPINFOHEADER {
        biSize: size_of::<BITMAPINFOHEADER>() as u32,
        biWidth: width as i32,
        biHeight: -(height as i32),
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

pub unsafe fn update_layered(win_handle: HWND, mem_dc: HDC, width: u32, height: u32) {
    let screen_dc = unsafe { GetDC(HWND(0)) };

    let size = SIZE {
        cx: width as i32,
        cy: height as i32,
    };

    let screen_width = unsafe { GetSystemMetrics(SM_CXSCREEN) };
    let screen_height = unsafe { GetSystemMetrics(SM_CYSCREEN) };

    let center_position = POINT {
        x: (screen_width - width as i32) / 2,
        y: (screen_height - height as i32) / 2,
    };

    let zero_position = POINT { x: 0, y: 0 };

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
            Some(&center_position),
            Some(&size),
            mem_dc,
            Some(&zero_position),
            COLORREF(0),
            Some(&blend),
            ULW_ALPHA,
        )
        .unwrap();
        ReleaseDC(HWND(0), screen_dc)
    };
}

pub fn clear(hwnd: HWND) {
    unsafe {
        let hdc_screen = GetDC(HWND(0));
        let mem_dc = CreateCompatibleDC(hdc_screen);
        ReleaseDC(HWND(0), hdc_screen);

        let size = SIZE { cx: 1, cy: 1 };
        let pos = POINT { x: 0, y: 0 };
        let src = POINT { x: 0, y: 0 };

        // 1×1 pixel fully transparent black
        let pixel = [0u8, 0u8, 0u8, 0u8];

        let bitmap_info = BITMAPINFO {
            bmiHeader: BITMAPINFOHEADER {
                biSize: std::mem::size_of::<BITMAPINFOHEADER>() as u32,
                biWidth: 1,
                biHeight: -1, // top-down
                biPlanes: 1,
                biBitCount: 32,
                biCompression: BI_RGB.0,
                ..Default::default()
            },
            ..Default::default()
        };

        let mut ptr: *mut std::ffi::c_void = std::ptr::null_mut();

        let bmp =
            CreateDIBSection(mem_dc, &bitmap_info, DIB_RGB_COLORS, &mut ptr, None, 0).unwrap();

        std::ptr::copy_nonoverlapping(pixel.as_ptr(), ptr as *mut u8, 4);

        let old = SelectObject(mem_dc, bmp);

        let blend = BLENDFUNCTION {
            BlendOp: AC_SRC_OVER as u8,
            BlendFlags: 0,
            SourceConstantAlpha: 255,
            AlphaFormat: AC_SRC_ALPHA as u8,
        };

        let hdc_screen2 = GetDC(HWND(0));
        UpdateLayeredWindow(
            hwnd,
            hdc_screen2,
            Some(&pos),
            Some(&size),
            mem_dc,
            Some(&src),
            COLORREF(0),
            Some(&blend),
            ULW_ALPHA,
        )
        .unwrap();

        ReleaseDC(HWND(0), hdc_screen2);

        SelectObject(mem_dc, old);
        DeleteDC(mem_dc);
    }
}

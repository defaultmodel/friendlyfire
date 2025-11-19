use std::{ffi::c_void, io::Cursor, mem::size_of, ptr};

use image::ImageReader;
use windows::Win32::{Foundation::*, Graphics::Gdi::*, UI::WindowsAndMessaging::*};

use super::splash_window::Window;

pub fn update_image(win: &Window, bytes: &[u8]) {
    // ---------- decode PNG as RGBA ----------
    let decoded = ImageReader::new(Cursor::new(bytes))
        .with_guessed_format()
        .unwrap()
        .decode()
        .unwrap()
        .to_rgba8();

    let (width, height) = decoded.dimensions();
    let rgba = decoded.into_raw();

    // ---------- convert to premultiplied BGRA ----------
    let bgra = rgba_to_premultiplied_bgra(&rgba);

    unsafe {
        // ---------- prepare memory DC & bitmap ----------
        let hdc_screen = GetDC(HWND(0));
        let mem_dc = CreateCompatibleDC(GetDC(HWND(0)));
        ReleaseDC(HWND(0), hdc_screen);

        let bitmap_info = BITMAPINFO {
            bmiHeader: BITMAPINFOHEADER {
                biSize: size_of::<BITMAPINFOHEADER>() as u32,
                biWidth: width as i32,
                biHeight: -(height as i32), // A top-down DIB, in which the origin lies at the upper-left corner.
                biPlanes: 1,
                biBitCount: 32,
                biCompression: BI_RGB.0,
                ..Default::default()
            },
            ..Default::default()
        };

        let mut bitmap_ptr: *mut c_void = ptr::null_mut();

        let dib = CreateDIBSection(
            mem_dc,
            &bitmap_info,
            DIB_RGB_COLORS,
            &mut bitmap_ptr,
            None,
            0,
        )
        .unwrap();

        // copy BGRA pixels → DIB buffer
        ptr::copy_nonoverlapping(bgra.as_ptr(), bitmap_ptr as *mut u8, bgra.len());

        let old = SelectObject(mem_dc, dib);

        // ---------- prepare UpdateLayeredWindow params ----------
        let size = SIZE {
            cx: width as i32,
            cy: height as i32,
        };

        let pos = POINT { x: 0, y: 0 };

        let src = POINT { x: 0, y: 0 };

        // QUESTION: What will this do if the image as not alpha channel ?
        let blend = BLENDFUNCTION {
            BlendOp: AC_SRC_OVER as u8,
            BlendFlags: 0,
            SourceConstantAlpha: 255,
            AlphaFormat: AC_SRC_ALPHA as u8,
        };

        // ---------- Update the layered window ----------
        let hdc_screen_again = GetDC(HWND(0));
        UpdateLayeredWindow(
            win.handle,
            hdc_screen_again,
            Some(&pos),
            Some(&size),
            mem_dc,
            Some(&src),
            COLORREF::default(),
            Some(&blend),
            ULW_ALPHA,
        )
        .unwrap();
        ReleaseDC(HWND(0), hdc_screen_again);

        // cleanup
        SelectObject(mem_dc, old);
        DeleteDC(mem_dc);
        ReleaseDC(HWND(0), hdc_screen_again);

        // if res.is_err() {
        //     eprintln!("UpdateLayeredWindow failed: {:?}", GetLastError());
        //     res.unwrap();
        // }
    }
}

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

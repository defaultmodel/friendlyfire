use std::{ffi::c_void, io::Cursor};

use image::ImageReader;
use windows::Win32::Graphics::{
    Direct2D::{Common::*, *},
    Dxgi::Common::*,
};

use super::splash_window::Window;

pub fn create_factory() -> ID2D1Factory {
    unsafe {
        D2D1CreateFactory(
            D2D1_FACTORY_TYPE_MULTI_THREADED,
            Some(&D2D1_FACTORY_OPTIONS {
                debugLevel: D2D1_DEBUG_LEVEL_NONE,
            }),
        )
        .unwrap()
    }
}

fn transparent_color() -> D2D1_COLOR_F {
    D2D1_COLOR_F {
        r: 0.0,
        g: 0.0,
        b: 0.0,
        a: 0.0,
    }
}

pub fn update_image(win: &Window, bytes: &[u8]) {
    let decoded = ImageReader::new(Cursor::new(bytes))
        .with_guessed_format()
        .unwrap()
        .decode()
        .unwrap()
        .to_rgba8();

    let (width, height) = decoded.dimensions();
    let raw = decoded.into_raw();

    let bgra = rgba_to_premultiplied_bgra(&raw);

    // lazy init render target
    let mut render_target_lock = win.render_target.lock().unwrap();
    if render_target_lock.is_none() {
        *render_target_lock = Some(create_hwnd_render_target(
            &win.direct2d_factory,
            win.handle,
            width,
            height,
        ));
    }

    let render_target = render_target_lock.as_ref().unwrap();

    // ---------- create bitmap ----------
    let mut bitmap_lock = win.bitmap.lock().unwrap();
    *bitmap_lock = Some(create_bitmap(render_target, &bgra, width, height));

    let bitmap = bitmap_lock.as_ref().unwrap();

    // ---------- draw ----------
    unsafe {
        render_target.BeginDraw();
        render_target.Clear(Some(&transparent_color()));
        render_target.DrawBitmap(
            bitmap,
            None,
            1.0,
            D2D1_BITMAP_INTERPOLATION_MODE_LINEAR,
            None,
        );
        render_target.EndDraw(None, None).unwrap();
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

pub fn create_hwnd_render_target(
    factory: &ID2D1Factory,
    hwnd: windows::Win32::Foundation::HWND,
    width: u32,
    height: u32,
) -> ID2D1HwndRenderTarget {
    let props = D2D1_RENDER_TARGET_PROPERTIES {
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

    let hwnd_props = D2D1_HWND_RENDER_TARGET_PROPERTIES {
        hwnd,
        pixelSize: D2D_SIZE_U { width, height },
        presentOptions: D2D1_PRESENT_OPTIONS_NONE,
    };

    unsafe {
        factory
            .CreateHwndRenderTarget(&props, &hwnd_props)
            .expect("Failed to create render target")
    }
}

pub fn create_bitmap(
    rt: &ID2D1HwndRenderTarget,
    data: &[u8],
    width: u32,
    height: u32,
) -> ID2D1Bitmap {
    let props = D2D1_BITMAP_PROPERTIES {
        pixelFormat: D2D1_PIXEL_FORMAT {
            format: DXGI_FORMAT_B8G8R8A8_UNORM,
            alphaMode: D2D1_ALPHA_MODE_PREMULTIPLIED,
        },
        dpiX: 96.0,
        dpiY: 96.0,
    };

    unsafe {
        rt.CreateBitmap(
            D2D_SIZE_U { width, height },
            Some(data.as_ptr() as *const c_void),
            4 * width,
            &props,
        )
        .unwrap()
    }
}

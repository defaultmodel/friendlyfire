use windows::{
    Win32::{Graphics::Direct2D::*, System::Com::*},
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

fn main() -> Result<()> {
    unsafe {
        CoInitializeEx(None, COINIT_APARTMENTTHREADED).unwrap();
        let factory = init_direct2d()?;
        println!("Direct2D factory created!");
        dbg!(factory);
        CoUninitialize();
    }
    Ok(())
}

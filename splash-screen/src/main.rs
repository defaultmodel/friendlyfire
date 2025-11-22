use std::fs;

use friendlyfire_shared_lib::{Message, MessageType};
use windows::Win32::Foundation::HWND;
use windows::Win32::UI::WindowsAndMessaging::{DispatchMessageA, GetMessageA, TranslateMessage};

use crate::media::decoder::MediaDecoder;
use crate::window::{PlatformSplashWindow, SplashWindow};

mod media;
mod network;
mod window;

fn main() -> anyhow::Result<()> {
    // 1) Create the window, do not show yet
    let window = PlatformSplashWindow::new();

    let img_bytes = fs::read("bonk.png").expect("bonk.png not found");
    let message = Message {
        version: "1.0.0".to_string(),
        kind: MessageType::ShowImage { bytes: img_bytes },
        party: Some("test-party".to_string()),
    };

    match message.kind {
        MessageType::ShowImage { bytes } => {
            let decoded_media = MediaDecoder::decode(&bytes).unwrap();
            window.show_media(decoded_media)
        }

        MessageType::ShowVideo { bytes } => {
            let decoded_media = MediaDecoder::decode(&bytes).unwrap();
            window.show_media(decoded_media)
        }

        MessageType::Clear => {
            window.clear();
        }
    }

    // Basic Win32 message loop
    unsafe {
        let mut msg = std::mem::zeroed();
        while GetMessageA(&mut msg, HWND(0), 0, 0).into() {
            let _ = TranslateMessage(&msg);
            DispatchMessageA(&msg);
        }
    }

    // 2) Connect to the server
    // let mut client = WSClient::connect("wss://example.com/party");

    // println!("Connected. Waiting for messages…");

    // 3) Main receive loop
    // while let Some(rmp) = client.recv() {
    //     let msg: Message = rmp_serde::from_slice(rmp)?;

    //     match msg.kind {
    //         MessageType::ShowImage { bytes } => {
    //             let decoded_media = MediaDecoder::decode(&bytes).unwrap();
    //             window.show_media(decoded_media)
    //         }

    //         MessageType::ShowVideo { bytes } => {
    //             let decoded_media = MediaDecoder::decode(&bytes).unwrap();
    //             window.show_media(decoded_media)
    //         }

    //         MessageType::Clear => {
    //             window.clear();
    //         }
    //     }
    // }

    // unsafe { CoInitializeEx(None, COINIT_APARTMENTTHREADED).unwrap() };

    // let window = window::Window::new();
    // let image_bytes = fs::read("bonk.png")?;
    // window.update_image(&image_bytes);

    // window.show();

    // // Basic Win32 message loop
    // unsafe {
    //     let mut msg = std::mem::zeroed();
    //     while windows::Win32::UI::WindowsAndMessaging::GetMessageA(
    //         &mut msg,
    //         windows::Win32::Foundation::HWND(0),
    //         0,
    //         0,
    //     )
    //     .into()
    //     {
    //         let _ = TranslateMessage(&msg);
    //         DispatchMessageA(&msg);
    //     }
    // }

    // unsafe { CoInitializeEx(None, COINIT_APARTMENTTHREADED).unwrap() };
    Ok(())
}

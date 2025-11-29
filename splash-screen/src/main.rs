use std::fs;

use crate::{
    compositor::Compositor,
    overlay::{AnimatedOverlay, OverlayImage},
    window::{SplashWindow, Win32Renderer, Win32Window},
};

use friendlyfire_shared_lib::{DisplayOptions, Message, MessageType, Overlay as LibOverlay};
use windows::Win32::{
    Foundation::HWND,
    UI::WindowsAndMessaging::{DispatchMessageA, GetMessageA, TranslateMessage},
};

mod compositor;
mod frame;

mod overlay;
mod window;

fn receive_mock_message() -> Message {
    Message {
        version: "1.0.0".to_string(),
        party: friendlyfire_shared_lib::Party {
            id: "beepboop".to_string(),
            name: "FriendlyParty".to_string(),
        },
        kind: MessageType::ShowMedia {
            overlays: vec![LibOverlay::AnimatedImage {
                bytes: fs::read("john-walk.gif").unwrap(),
                x: 200,
                y: 200,
                z_index: 1000,
            }],
            options: DisplayOptions { timeout_ms: 3000 },
        },
    }
}

fn main() -> anyhow::Result<()> {
    let mut window = Win32Window::create()?;
    window.show();

    // Mock incoming message
    let message: Message = receive_mock_message();

    // Create compositor
    let mut compositor = Compositor::new(1920, 1080);

    // Translate message overlays into internal overlays
    if let MessageType::ShowMedia { overlays, .. } = message.kind {
        for overlay in overlays {
            match overlay {
                LibOverlay::Image {
                    bytes,
                    x,
                    y,
                    z_index,
                } => {
                    let overlay = OverlayImage::from_bytes(&bytes, x, y, z_index);
                    compositor.add_overlay(Box::new(overlay));
                }
                LibOverlay::AnimatedImage {
                    bytes,
                    x,
                    y,
                    z_index,
                } => {
                    let overlay = AnimatedOverlay::from_bytes(&bytes, x, y, z_index, 0);
                    compositor.add_overlay(Box::new(overlay));
                }
                LibOverlay::Text {
                    text,
                    size,
                    color,
                    x,
                    y,
                    z_index,
                } => {
                    todo!();
                }
            }
        }
    }

    // Compose a single frame at timestamp 0
    let canvas = compositor.render(0);

    // Render it once
    window.draw_frame(canvas);

    // Basic Win32 message loop
    unsafe {
        let mut msg = std::mem::zeroed();
        while GetMessageA(&mut msg, HWND(0), 0, 0).into() {
            let _ = TranslateMessage(&msg);
            DispatchMessageA(&msg);
        }
    }
    Ok(())
}

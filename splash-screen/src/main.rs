use std::{fs, time};

use crate::{
    compositor::Compositor,
    overlay::{AnimatedOverlay, OverlayImage},
    window::{SplashWindow, Win32Renderer, Win32Window},
};

use friendlyfire_shared_lib::{DisplayOptions, Message, MessageType, Overlay as LibOverlay};
use tokio::time::Instant;

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
                bytes: fs::read("jonh-walk.gif").unwrap(),
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
                    let start_time_ms = Instant::now().elapsed().as_millis() as u64; // will be near 0 but fine
                    let overlay = AnimatedOverlay::from_bytes(&bytes, x, y, z_index, start_time_ms);
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
    // Use a monotonic clock origin
    let origin = Instant::now();

    loop {
        let timestamp_ms = origin.elapsed().as_millis() as u64;

        let canvas = compositor.render(timestamp_ms);
        window.draw_frame(canvas);

        // how long until next per-overlay frame change?
        let until_next = compositor.time_until_next_frame_ms(timestamp_ms);
        let sleep_ms = until_next.unwrap_or(1); // fallback 60 fps

        // Sleep exactly the requested amount (renderer controls timing)
        std::thread::sleep(time::Duration::from_millis(sleep_ms));
    }
}

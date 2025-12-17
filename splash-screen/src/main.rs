use std::{fs, str::FromStr, time::Duration};

use crate::{
    compositor::Compositor,
    overlay::{AnimatedOverlay, ImageOverlay, TextOverlay},
    window::{SplashWindow, Win32Renderer, Win32Window},
};

use cosmic_text::{FontSystem, SwashCache};
use ff::{
    DisplayOptions, Overlay as LibOverlay, SenderInfo, ServerMessage, ServerMessageType, Version,
};
use tokio::time::Instant;
use uuid::Uuid;

mod compositor;
mod frame;

mod overlay;
mod window;

fn receive_mock_message() -> ServerMessage {
    ServerMessage {
        version: Version::from_str("0.1.0").unwrap(),
        sender: SenderInfo { id: Uuid::new_v4() },
        kind: ServerMessageType::Overlays {
            overlays: vec![
                LibOverlay::AnimatedImage {
                    bytes: fs::read("john-walk.gif").unwrap(),
                    offset_left: 800,
                    offset_top: 0,
                    z_index: 1000,
                },
                LibOverlay::Image {
                    bytes: fs::read("bonk.png").unwrap(),
                    offset_left: 0,
                    offset_top: 0,
                    z_index: 1010,
                },
                LibOverlay::Text {
                    text: "Zoubida!".to_string(),
                    size: 52,
                    color: [255, 255, 255, 255],
                    offset_left: 0,
                    offset_top: 0,
                    z_index: 1020,
                },
            ],
            options: DisplayOptions { timeout_ms: 3000 },
        },
    }
}

pub fn add_overlays_from_message(
    compositor: &mut Compositor,
    font_system: &mut FontSystem,
    swash_cache: &mut SwashCache,
    message: ServerMessageType,
) -> anyhow::Result<()> {
    if let ServerMessageType::Overlays { overlays, .. } = message {
        for overlay in overlays {
            match overlay {
                LibOverlay::Image {
                    bytes,
                    offset_left,
                    offset_top,
                    z_index,
                } => {
                    compositor.add_overlay(Box::new(ImageOverlay::from_bytes(
                        &bytes,
                        offset_left,
                        offset_top,
                        z_index,
                    )));
                }

                LibOverlay::AnimatedImage {
                    bytes,
                    offset_left,
                    offset_top,
                    z_index,
                } => {
                    let t0 = Instant::now().elapsed().as_millis() as u64;
                    compositor.add_overlay(Box::new(AnimatedOverlay::from_bytes(
                        &bytes,
                        offset_left,
                        offset_top,
                        z_index,
                        t0,
                    )));
                }

                LibOverlay::Text {
                    text,
                    size,
                    color,
                    offset_left,
                    offset_top,
                    z_index,
                } => {
                    compositor.add_overlay(Box::new(TextOverlay::from_bytes(
                        font_system,
                        swash_cache,
                        &text,
                        size,
                        &color,
                        offset_left,
                        offset_top,
                        z_index as i32,
                    )?));
                }
            }
        }
    }

    Ok(())
}

pub async fn run_render_loop(window: &mut Win32Window, compositor: &mut Compositor) {
    let origin = Instant::now();

    loop {
        let timestamp_ms = origin.elapsed().as_millis() as u64;
        let frame = compositor.render(timestamp_ms);
        window.draw_frame(frame);

        let delay = compositor
            .time_until_next_frame_ms(timestamp_ms)
            .unwrap_or(200);

        tokio::time::sleep(Duration::from_millis(delay)).await;
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut font_system = FontSystem::new();
    let mut swash_cache = SwashCache::new();

    let mut window = Win32Window::create()?;
    window.show();

    let (w, h) = window.dimensions();
    let mut compositor = Compositor::new(w, h);

    let message = receive_mock_message();
    add_overlays_from_message(
        &mut compositor,
        &mut font_system,
        &mut swash_cache,
        message.kind,
    )?;

    run_render_loop(&mut window, &mut compositor).await;

    Ok(())
}

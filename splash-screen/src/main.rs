use std::{
    fs,
    time::{self, Duration},
};

use crate::{
    compositor::Compositor,
    overlay::{AnimatedOverlay, ImageOverlay, TextOverlay},
    window::{SplashWindow, Win32Renderer, Win32Window},
};

use cosmic_text::{FontSystem, SwashCache};
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
            overlays: vec![
                LibOverlay::AnimatedImage {
                    bytes: fs::read("jonh-walk.gif").unwrap(),
                    x: 800,
                    y: 0,
                    z_index: 1000,
                },
                LibOverlay::Image {
                    bytes: fs::read("bonk.png").unwrap(),
                    x: 0,
                    y: 0,
                    z_index: 1010,
                },
                LibOverlay::Text {
                    text: "Zoubida!".to_string(),
                    size: 52,
                    color: [255, 255, 255, 255],
                    x: 0,
                    y: 0,
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
    message: MessageType,
) -> anyhow::Result<()> {
    if let MessageType::ShowMedia { overlays, .. } = message {
        for overlay in overlays {
            match overlay {
                LibOverlay::Image {
                    bytes,
                    x,
                    y,
                    z_index,
                } => {
                    compositor
                        .add_overlay(Box::new(ImageOverlay::from_bytes(&bytes, x, y, z_index)));
                }

                LibOverlay::AnimatedImage {
                    bytes,
                    x,
                    y,
                    z_index,
                } => {
                    let t0 = Instant::now().elapsed().as_millis() as u64;
                    compositor.add_overlay(Box::new(AnimatedOverlay::from_bytes(
                        &bytes, x, y, z_index, t0,
                    )));
                }

                LibOverlay::Text {
                    text,
                    size,
                    color,
                    x,
                    y,
                    z_index,
                } => {
                    compositor.add_overlay(Box::new(TextOverlay::from_bytes(
                        font_system,
                        swash_cache,
                        &fs::read("fonts/AtkinsonHyperlegibleNextVF-Variable.ttf")?,
                        &text,
                        size,
                        &color,
                        x,
                        y,
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

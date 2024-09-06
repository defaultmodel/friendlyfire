use image::ImageReader;
use play::image::{pick_image, ImagePayload};
use play::upload_file;
use play::video::{pick_video, VideoPayload};
use tauri::{AppHandle, Manager, Url};
use tauri_plugin_log::fern::colors::ColoredLevelConfig;
use tokio_tungstenite::tungstenite::Message;
use ws::close::close_ws_connection;
use ws::init::init_ws_connection;
use ws::messages::send_ws_message;

pub mod play;
pub mod ws;

pub fn run() {
    tauri::Builder::default()
        .plugin(
            tauri_plugin_log::Builder::new()
                .with_colors(ColoredLevelConfig::default())
                .level(log::LevelFilter::Debug)
                .build(),
        )
        .plugin(tauri_plugin_websocket::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            join_server,
            leave_server,
            send_ws_string,
            play_image,
            play_video,
            test_command
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
async fn join_server(handle: AppHandle) {
    init_ws_connection(handle).await.unwrap();
    // From here WS_CONNECTION is set
    // After this client receives joined message and updates the client count
}

#[tauri::command]
async fn leave_server() {
    close_ws_connection().await.unwrap();
}

#[tauri::command]
async fn send_ws_string(message: String) {
    send_ws_message(Message::Text(message)).await.unwrap();
}

#[tauri::command]
async fn play_image(handle: AppHandle, text: String) {
    if let Some(file) = pick_image(&handle) {
        let img = ImageReader::open(&file.path).unwrap().decode().unwrap();
        let width = img.width() as f64;
        let height = img.height() as f64;
        let remote_path = upload_file(file).await;
        let remote_path =
            Url::parse(format!("http://localhost:3000/uploads/{}", remote_path).as_str()).unwrap();
        let payload = ImagePayload::new(remote_path, text.clone(), width, height);
        payload.send().await;
    }
}

#[tauri::command]
async fn play_video(handle: AppHandle, text: String) {
    if let Some(file) = pick_video(&handle) {
        let remote_path = upload_file(file).await;
        let remote_path =
            Url::parse(format!("http://localhost:3000/uploads/{}", remote_path).as_str()).unwrap();
        let payload = VideoPayload::new(remote_path, text.clone());
        payload.send().await;
    }
}

#[tauri::command]
async fn test_command(handle: AppHandle) {
    handle.get_webview_window("main").unwrap().hide().unwrap();
}

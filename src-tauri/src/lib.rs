mod internal;

use internal::audio::detect_virtual_mic;
use internal::cmd::{
    add_song, download_from_myinstants, list_audio_devices, list_songs, play_audio, stop_all_sounds,
};
use internal::state::AppState;
use std::sync::Mutex;
use tauri::Manager;
use tauri_plugin_dialog::{DialogExt, MessageDialogKind};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .setup(|app| {
            if let Some(virtual_mic) = detect_virtual_mic() {
                println!("Virtual mic detected: {}", virtual_mic);
                app.manage(Mutex::new(AppState::default()));
                let state = app.state::<Mutex<AppState>>();

                let mut state = state.lock().unwrap();
                state.virtual_mic = virtual_mic;

            } else {
                eprintln!("Could not find a virtual mic");
                let _ = app
                    .dialog()
                    .message("You do not have VB-Audio installed. Please install it from https://vb-audio.com/Cable/")
                    .kind(MessageDialogKind::Error)
                    .title("Error")
                    .blocking_show();
                app.handle().exit(1);
            }
            Ok(())
        })
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            play_audio,
            list_audio_devices,
            stop_all_sounds,
            add_song,
            list_songs,
            download_from_myinstants
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

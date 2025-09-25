mod internal;

use internal::audio::detect_virtual_mic;
use internal::cmd::{
    add_song, download_from_myinstants, list_audio_devices, list_songs, play_audio, stop_all_sounds,
};
use internal::recorder::{start_recording, stop_recording};
use internal::state::AppState;
use std::sync::Mutex;
use std::time::Duration;
use tauri::Manager;
use tauri_plugin_dialog::{DialogExt, MessageDialogKind};
use tauri_plugin_svelte::PrettyTomlMarshaler;
use tauri_store::SaveStrategy;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .setup(|app| {
            if let Some(virtual_mic) = detect_virtual_mic() {
                println!("Virtual mic detected: {}", &virtual_mic);
                app.manage(AppState {
                    virtual_mic: Mutex::new(virtual_mic),
                    recorder: Mutex::new(None),
                });
            } else {
                eprintln!("Could not find a virtual mic");
                let _ = app
                    .dialog()
                    .message(
                        "You do not have VB-Audio installed. Please install it from https://vb-audio.com/Cable/",
                    )
                    .kind(MessageDialogKind::Error)
                    .title("Error")
                    .blocking_show();
                app.handle().exit(1);
            }
            Ok(())
        })
        .plugin(tauri_plugin_svelte::Builder::new()
        .marshaler(Box::new(PrettyTomlMarshaler))
            .default_save_strategy(SaveStrategy::throttle_secs(3))
            .autosave(Duration::from_secs(60))
            .build()
        )
        .plugin(
            tauri_plugin_log::Builder::new()
            .target(tauri_plugin_log::Target::new(
                tauri_plugin_log::TargetKind::LogDir {
                    file_name: Some("logs".to_string()),
                },
            ))
            .build()
        )
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
            start_recording,
            stop_recording,
            download_from_myinstants
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

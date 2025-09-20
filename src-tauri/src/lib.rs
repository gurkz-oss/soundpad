// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use cpal::traits::{DeviceTrait, HostTrait};
use rodio::Sink;
use serde::Serialize;
use std::error::Error;
use std::path::PathBuf;
use std::sync::{Arc, Mutex, OnceLock};
use tauri::Manager;
use tauri_plugin_updater::UpdaterExt;

#[derive(Serialize)]
struct SongInfo {
    name: String,
    path: String,
}

static ACTIVE_SINKS: OnceLock<Mutex<Vec<Arc<Sink>>>> = OnceLock::new();

fn get_active_sinks() -> &'static Mutex<Vec<Arc<Sink>>> {
    ACTIVE_SINKS.get_or_init(|| Mutex::new(Vec::new()))
}

fn detect_virtual_mic() -> Option<String> {
    let host = cpal::default_host();
    let devices = host.output_devices().unwrap(); // or output_devices if needed

    for device in devices {
        if let Ok(name) = device.name() {
            println!("device name is {}", name);
            // Skip devices containing "16ch"
            if name.contains("16ch") {
                continue;
            }

            if name.contains("VoiceMeeter") || name.contains("CABLE") {
                println!("Virtual mic detected: {}", name);
                return Some(name);
            }
        }
    }

    println!("Virtual mic not detected. Install VB-Cable or VoiceMeeter.");
    None
}

#[tauri::command]
fn list_audio_devices() -> Vec<String> {
    let host = cpal::default_host();
    let devices = host.output_devices().unwrap();
    let mut device_names = vec![];
    for device in devices {
        if let Ok(name) = device.name() {
            device_names.push(name);
        }
    }
    device_names
}

pub async fn play(path: String) -> Result<bool, Box<dyn Error>> {
    let stream_handle = rodio::OutputStreamBuilder::open_default_stream()?;
    let sink = rodio::Sink::connect_new(stream_handle.mixer());

    let file = std::fs::File::open(path)?;
    sink.append(rodio::Decoder::try_from(file)?);

    sink.sleep_until_end();

    println!("play finished");

    Ok(true)
}

fn new_play(path: String, device_name: String) -> Result<String, Box<dyn std::error::Error>> {
    let host = cpal::default_host();

    // Helper to create a sink for a device
    fn create_sink_for_device(
        host: &cpal::Host,
        device_name: &str,
        path: &str,
    ) -> Result<Arc<rodio::Sink>, Box<dyn std::error::Error>> {
        let device = host
            .output_devices()?
            .find(|d| d.name().unwrap_or_default() == device_name)
            .ok_or(format!("Device '{}' not found", device_name))?;

        let stream_handle =
            rodio::OutputStreamBuilder::from_device(device)?.open_stream_or_fallback()?;

        let sink = Arc::new(rodio::Sink::connect_new(stream_handle.mixer()));

        let file = std::fs::File::open(path)?;
        let source = rodio::Decoder::new(std::io::BufReader::new(file))?;
        sink.append(source);

        // Keep sink and stream_handle alive in a thread
        let sink_clone = Arc::clone(&sink);
        std::thread::spawn(move || {
            let _stream_handle = stream_handle;
            sink_clone.sleep_until_end();
        });

        Ok(sink)
    }

    // 1. Play on the user-selected device
    let sink1 = create_sink_for_device(&host, &device_name, &path)?;
    get_active_sinks().lock().unwrap().push(Arc::clone(&sink1));

    // 2. Detect virtual mic and play on it if present
    if let Some(virtual_name) = detect_virtual_mic() {
        let sink2 = create_sink_for_device(&host, &virtual_name, &path)?;
        get_active_sinks().lock().unwrap().push(Arc::clone(&sink2));
    }

    Ok("Playing audio on both devices...".into())
}

#[tauri::command]
fn stop_all_sounds() -> String {
    let mut sinks = get_active_sinks().lock().unwrap();
    for sink in sinks.iter() {
        sink.stop();
    }
    sinks.clear();
    "All sounds stopped.".into()
}

#[tauri::command]
fn play_audio(path: String, device_name: String) -> Result<String, ()> {
    match new_play(path, device_name) {
        Ok(msg) => Ok(msg),
        Err(err) => {
            eprintln!("Error playing audio: {}", err);
            Err(())
        }
    }
}

#[tauri::command]
fn add_song(path: String, app_handle: tauri::AppHandle) -> Result<(), String> {
    let data_dir: PathBuf = app_handle
        .path()
        .app_data_dir()
        .map_err(|e| format!("Failed to find app data directory: {e}"))?;

    let songs_dir = data_dir.join("songs");
    std::fs::create_dir_all(&songs_dir)
        .map_err(|e| format!("Could not create songs directory: {e}"))?;

    let src = PathBuf::from(&path);
    let dest = songs_dir.join(
        src.file_name()
            .ok_or_else(|| "Invalid source file name".to_string())?,
    );

    std::fs::copy(&src, &dest).map_err(|e| format!("Failed to copy file: {e}"))?;

    println!("Song copied to {}", dest.display());
    Ok(())
}

#[tauri::command]
fn list_songs(app_handle: tauri::AppHandle) -> Result<Vec<SongInfo>, String> {
    let songs_dir: PathBuf = app_handle
        .path()
        .app_data_dir()
        .map_err(|e| format!("Failed to find app data directory: {e}"))?
        .join("songs");

    if !songs_dir.exists() {
        return Ok(Vec::new());
    }

    let mut files = Vec::new();
    for entry in
        std::fs::read_dir(&songs_dir).map_err(|e| format!("Failed to read songs directory: {e}"))?
    {
        let entry = entry.map_err(|e| format!("Error reading entry: {e}"))?;
        let path = entry.path();

        if entry
            .file_type()
            .map_err(|e| format!("File type error: {e}"))?
            .is_file()
        {
            // Extract file name without extension
            let name = path
                .file_stem()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string();

            files.push(SongInfo {
                name,
                path: path.to_string_lossy().to_string(),
            });
        }
    }

    Ok(files)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
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
            list_songs
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

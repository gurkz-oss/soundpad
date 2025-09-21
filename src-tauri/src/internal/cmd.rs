use crate::internal::state::AppState;
use crate::internal::{audio, fs, myinstants, util};
use cpal::traits::{DeviceTrait, HostTrait};
use std::sync::Arc;
use std::time::SystemTime;
use std::{path::PathBuf, sync::Mutex};
use tauri::{Manager, State};

#[tauri::command]
pub fn list_audio_devices() -> Vec<String> {
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

fn new_play(
    path: String,
    device_name: String,
    state: State<'_, Mutex<AppState>>,
) -> Result<String, Box<dyn std::error::Error>> {
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
    audio::get_active_sinks()
        .lock()
        .unwrap()
        .push(Arc::clone(&sink1));

    let state = state.lock().unwrap();

    // 2. Detect virtual mic and play on it if present
    let sink2 = create_sink_for_device(&host, &state.virtual_mic, &path)?;
    audio::get_active_sinks()
        .lock()
        .unwrap()
        .push(Arc::clone(&sink2));

    Ok("Playing audio on both devices...".into())
}

#[tauri::command]
pub fn stop_all_sounds() -> String {
    let mut sinks = audio::get_active_sinks().lock().unwrap();
    for sink in sinks.iter() {
        sink.stop();
    }
    sinks.clear();
    "All sounds stopped.".into()
}

#[tauri::command]
pub fn play_audio(
    path: String,
    device_name: String,
    state: State<'_, Mutex<AppState>>,
) -> Result<String, ()> {
    match new_play(path, device_name, state) {
        Ok(msg) => Ok(msg),
        Err(err) => {
            eprintln!("Error playing audio: {}", err);
            Err(())
        }
    }
}

#[tauri::command]
pub fn add_song(path: String, app_handle: tauri::AppHandle) -> Result<(), String> {
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
pub fn list_songs(app_handle: tauri::AppHandle) -> Result<Vec<audio::SongInfo>, String> {
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

            // Get the file's creation time
            let created_time = match std::fs::metadata(&path) {
                Ok(metadata) => match metadata.created() {
                    Ok(time) => time,
                    Err(_) => SystemTime::UNIX_EPOCH, // Fallback in case creation time is unavailable
                },
                Err(_) => SystemTime::UNIX_EPOCH, // Fallback if we can't get metadata
            };

            files.push(audio::SongInfo {
                name,
                path: path.to_string_lossy().to_string(),
                created_time,
            });
        }
    }

    // Sort files by creation time, with the most recently created first
    files.sort_by(|a, b| b.created_time.cmp(&a.created_time));

    Ok(files)
}

#[tauri::command]
pub async fn download_from_myinstants(
    url: String,
    app_handle: tauri::AppHandle,
) -> Result<String, String> {
    // Fetch data from myinstants
    let data = myinstants::get_data(&url)
        .await
        .map_err(|e| e.to_string())?;

    // Construct file name
    let safe_name = util::sanitize_filename(&data.title);
    let name = format!("{}.mp3", safe_name);

    // Get app data directory
    let data_dir: PathBuf = app_handle
        .path()
        .app_data_dir()
        .map_err(|e| e.to_string())?;

    // Ensure songs directory exists
    let songs_dir = data_dir.join("songs");
    tokio::fs::create_dir_all(&songs_dir)
        .await
        .map_err(|e| e.to_string())?;

    // Destination path
    let dest = songs_dir.join(name);

    // Download the MP3
    fs::download_file(&data.mp3, &dest)
        .await
        .map_err(|e| e.to_string())?;

    println!("Song downloaded to {}", dest.display());
    Ok(dest.to_string_lossy().to_string())
}

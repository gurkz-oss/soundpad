use cpal::traits::{DeviceTrait, HostTrait};
use rodio::Sink;
use serde::Serialize;
use std::sync::{Arc, Mutex, OnceLock};
use std::time::SystemTime;

#[derive(Serialize)]
pub struct SongInfo {
    pub name: String,
    pub path: String,
    pub created_time: SystemTime,
}

static ACTIVE_SINKS: OnceLock<Mutex<Vec<Arc<Sink>>>> = OnceLock::new();

pub fn get_active_sinks() -> &'static Mutex<Vec<Arc<Sink>>> {
    ACTIVE_SINKS.get_or_init(|| Mutex::new(Vec::new()))
}

pub fn detect_virtual_mic() -> Option<String> {
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

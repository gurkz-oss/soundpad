#![cfg(target_os = "windows")]

use crate::internal::state::AppState;
use anyhow::Result;
use mp3lame_encoder::{Builder as Mp3Builder, DualPcm, FlushNoGap};
use std::{io::Write, path::PathBuf, thread, time::Duration};
use tauri::{AppHandle, Manager, State};
use wasapi::{initialize_mta, AudioClient, Direction, SampleType, StreamMode, WaveFormat};

pub struct RecorderHandle {
    stop_tx: std::sync::mpsc::Sender<()>,
    join: thread::JoinHandle<Result<PathBuf>>,
}

#[tauri::command]
pub fn start_recording(state: State<'_, AppState>, app_handle: AppHandle) -> Result<(), String> {
    let app_data_dir = app_handle
        .path()
        .app_data_dir()
        .map_err(|e| e.to_string())?;
    let mut guard = state.recorder.lock().unwrap();
    if guard.is_some() {
        return Err("Already recording".into());
    }

    // file we will write to (temp)
    let out_path = app_data_dir.join("songs").join(format!(
        "system_capture_{}.mp3",
        chrono::Utc::now().timestamp()
    ));

    let (tx, rx) = std::sync::mpsc::channel::<()>();

    // spawn capture + encode thread
    let join = thread::spawn(move || -> Result<PathBuf> {
        // init COM MTA (wasapi)
        let _ = initialize_mta();

        // create a loopback audio client for the current process (captures system audio)
        let pid = std::process::id();
        let mut audio_client =
            AudioClient::new_application_loopback_client(pid, /* include_tree */ false)?;

        // choose desired format; ask for 32-bit float and let the audio engine convert if needed
        let desired_format = WaveFormat::new(
            32usize,
            32usize,
            &SampleType::Float,
            44_100usize,
            2usize,
            None,
        );

        // Event-driven shared mode (20ms buffer)
        let mode = StreamMode::EventsShared {
            autoconvert: true,
            buffer_duration_hns: 200_000, // 20 ms
        };

        audio_client.initialize_client(&desired_format, &Direction::Capture, &mode)?;
        let capture_client = audio_client.get_audiocaptureclient()?;
        audio_client.start_stream()?; // begin capture

        // mp3 encoder builder
        let mut b = Mp3Builder::new().expect("lame builder");
        b.set_num_channels(2).expect("channels");
        b.set_sample_rate(44_100).expect("sample rate");
        b.set_brate(mp3lame_encoder::Bitrate::Kbps192)
            .expect("brate");
        b.set_quality(mp3lame_encoder::Quality::Best)
            .expect("quality");
        let mut encoder = b.build().expect("init lame");

        // Open output file and stream MP3 bytes as we encode
        let mut out_file = std::fs::File::create(&out_path)?;

        // We'll reuse a buffer for mp3 output each iteration
        let mut mp3_out_buf: Vec<u8> = Vec::new();

        // bytes per frame (block align)
        let bytes_per_frame = desired_format.get_blockalign() as usize;
        let channels = 2usize; // we asked for 2 channels

        // Big read buffer: allow up to e.g. 8192 frames per read (adjust if you like)
        let max_frames_per_read = 8192usize;
        let mut raw_buf = vec![0u8; max_frames_per_read * bytes_per_frame];

        loop {
            // stop check: non-blocking
            if let Ok(_) = rx.try_recv() {
                break;
            }

            // read from device (read_from_device will fill as much as available up to buffer size)
            let (frames_read, _bufinfo) = match capture_client.read_from_device(&mut raw_buf) {
                Ok((n, info)) => (n as usize, info),
                Err(e) => {
                    // non-fatal: sleep a bit and retry
                    eprintln!("wasapi read error: {:?}", e);
                    thread::sleep(Duration::from_millis(10));
                    continue;
                }
            };

            if frames_read == 0 {
                // nothing currently available
                thread::sleep(Duration::from_millis(5));
                continue;
            }

            // Convert raw bytes -> f32 samples (we requested float32). Then map to i16 and then to u16
            // Note: bytes_per_frame should be channels * 4 for float32
            let mut left: Vec<u16> = Vec::with_capacity(frames_read);
            let mut right: Vec<u16> = Vec::with_capacity(frames_read);

            for frame_idx in 0..frames_read {
                let base = frame_idx * bytes_per_frame;
                // safety: ensure indexes in bounds
                if base + 4 * channels > raw_buf.len() {
                    break;
                }
                let l_bytes = &raw_buf[base..base + 4];
                let r_bytes = &raw_buf[base + 4..base + 8];

                let l = f32::from_le_bytes([l_bytes[0], l_bytes[1], l_bytes[2], l_bytes[3]]);
                let r = f32::from_le_bytes([r_bytes[0], r_bytes[1], r_bytes[2], r_bytes[3]]);

                // convert [-1.0, 1.0] float -> i16 range, clamp to avoid overflow
                let li = (l * 32767.0).clamp(-32768.0, 32767.0) as i16;
                let ri = (r * 32767.0).clamp(-32768.0, 32767.0) as i16;

                // encoder API wants slices of u16 in this crate example; casting i16 -> u16 preserves bit pattern
                left.push(li as u16);
                right.push(ri as u16);
            }

            // encode chunk
            let input = DualPcm {
                left: left.as_slice(),
                right: right.as_slice(),
            };

            // reserve enough output space
            mp3_out_buf.reserve(mp3lame_encoder::max_required_buffer_size(input.left.len()));
            // encode into spare capacity, then increase length by encoded_size
            let encoded = encoder
                .encode(input, mp3_out_buf.spare_capacity_mut())
                .map_err(|e| anyhow::anyhow!("encode error: {:?}", e))?;
            unsafe {
                mp3_out_buf.set_len(mp3_out_buf.len().wrapping_add(encoded));
            }
            // write encoded bytes to file, then clear buffer for the next round
            out_file.write_all(&mp3_out_buf)?;
            mp3_out_buf.clear();
        }

        // final flush
        mp3_out_buf.reserve(1024);
        let flushed = encoder
            .flush::<FlushNoGap>(mp3_out_buf.spare_capacity_mut())
            .map_err(|e| anyhow::anyhow!("flush error: {:?}", e))?;
        unsafe {
            mp3_out_buf.set_len(mp3_out_buf.len().wrapping_add(flushed));
        }
        out_file.write_all(&mp3_out_buf)?;
        out_file.sync_all()?;

        Ok(out_path)
    });

    *guard = Some(RecorderHandle { stop_tx: tx, join });

    Ok(())
}

#[tauri::command]
pub fn stop_recording(state: State<'_, AppState>) -> Result<String, String> {
    let mut guard = state.recorder.lock().unwrap();
    if guard.is_none() {
        return Err("Not recording".into());
    }
    let rec = guard.take().unwrap();
    // notify thread to stop
    let _ = rec.stop_tx.send(());
    // join thread and get path
    match rec.join.join() {
        Ok(Ok(path)) => Ok(path.to_string_lossy().to_string()),
        Ok(Err(e)) => Err(format!("recording thread failed: {:?}", e)),
        Err(e) => Err(format!("join error: {:?}", e)),
    }
}

import { Song } from "$lib/songs.svelte";
import { invoke } from "@tauri-apps/api/core";

export async function load() {
  return {
    initialSongs: await invoke<Song[]>("list_songs"),
    initialDevices: await invoke<string[]>("list_audio_devices"),
  };
}

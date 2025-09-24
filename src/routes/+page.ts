import { invoke } from "@tauri-apps/api/core";
import type { PageLoad } from "./$types";

export const load: PageLoad = async () => {
  return {
    deviceList: await invoke<string[]>("list_audio_devices"),
  };
};

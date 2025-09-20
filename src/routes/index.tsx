import { createEffect, createResource, createSignal, onMount } from "solid-js";
import { invoke } from "@tauri-apps/api/core";
import { getVersion, getTauriVersion } from "@tauri-apps/api/app";
import { FileAdder } from "@/components/file-adder";
import { load } from "@tauri-apps/plugin-store";
import { appDataDir } from "@tauri-apps/api/path";
import { SongList } from "@/components/song-list";
import { selectedDevice, setSelectedDevice } from "..";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import { StatusButtons } from "@/components/status-buttons";

const store = await load("store.json");

const [devices, setDevices] = createSignal<string[]>([]);

export function HomePage() {
  const [appVersion] = createResource(() => getVersion());
  const [tauriVersion] = createResource(() => getTauriVersion());

  onMount(async () => {
    console.log(await appDataDir());

    const output = await store.get("speaker-output");

    if (typeof output === "string") {
      setSelectedDevice(output);
    }

    const deviceList = await invoke<string[]>("list_audio_devices");
    setDevices(deviceList);

    if (deviceList.length > 0 && !output) {
      setSelectedDevice(deviceList[0]);
    }
  });

  // Synchronize the selectedDevice signal with the store
  createEffect(async () => {
    const currentDevice = selectedDevice();
    if (currentDevice) {
      await store.set("speaker-output", currentDevice);
    }
  });

  return (
    <main class="p-2">
      <FileAdder />
      <p>
        app version: {appVersion()} and tauri version {tauriVersion()}
        selected device: {selectedDevice()}
      </p>
      {devices().length > 0 && (
        <>
          <label>Select device:</label>
          <Select
            onChange={setSelectedDevice}
            value={selectedDevice()}
            options={devices()}
            itemComponent={(props) => (
              <SelectItem item={props.item}>{props.item.rawValue}</SelectItem>
            )}
          >
            <SelectTrigger>
              <SelectValue<string>>
                {(state) => state.selectedOption()}
              </SelectValue>
            </SelectTrigger>
            <SelectContent />
          </Select>
        </>
      )}

      <SongList />

      <br />

      <StatusButtons />
    </main>
  );
}

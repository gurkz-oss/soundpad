import { createEffect, createSignal, onMount } from "solid-js";
import { invoke } from "@tauri-apps/api/core";
import { load } from "@tauri-apps/plugin-store";
import { appDataDir } from "@tauri-apps/api/path";
import { SongList } from "@/components/song-list";
import { selectedDevice, setSelectedDevice } from "@/libs/device";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import { StatusButtons } from "@/components/status-buttons";
import { SongAdder } from "@/components/song-adder";

const store = await load("store.json");

const [devices, setDevices] = createSignal<string[]>([]);

export function HomePage() {
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
    <>
      <SongAdder />
      {devices().length > 0 && (
        <>
          <Select
            required
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
    </>
  );
}

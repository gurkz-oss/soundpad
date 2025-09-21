import { selectedDevice, setSelectedDevice } from "@/libs/device";
import { invoke } from "@tauri-apps/api/core";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import { load } from "@tauri-apps/plugin-store";
import { createSignal, onMount, Show } from "solid-js";

const store = await load("store.json");

export function DeviceSelector() {
  const [devices, setDevices] = createSignal<string[]>([]);
  store.onKeyChange("speaker-output", (value) => {
    if (typeof value === "string") setSelectedDevice(value);
  });

  onMount(async () => {
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

  return (
    <Show when={devices().length > 0} fallback={<p>no devices found</p>}>
      <Select
        required
        onChange={async (state) => await store.set("speaker-output", state)}
        value={selectedDevice()}
        options={devices()}
        itemComponent={(props) => (
          <SelectItem item={props.item}>{props.item.rawValue}</SelectItem>
        )}
      >
        <SelectTrigger>
          <SelectValue<string>>{(state) => state.selectedOption()}</SelectValue>
        </SelectTrigger>
        <SelectContent />
      </Select>
    </Show>
  );
}

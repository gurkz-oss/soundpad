import { invoke } from "@tauri-apps/api/core";
import { Button } from "./ui/button";
import { open } from "@tauri-apps/plugin-dialog";
import { selectedDevice } from "..";

async function selectFile() {
  const path = await open({
    multiple: false,
    directory: false,
    filters: [
      {
        name: "mp3 files",
        extensions: ["mp3"],
      },
    ],
  });

  if (!selectedDevice()) return alert("Select an audio device");

  await invoke("play_audio", {
    path: path,
    deviceName: selectedDevice(),
  });
}

export function StatusButtons() {
  return (
    <div class="flex flex-row gap-2">
      <Button
        onClick={() => {
          selectFile();
        }}
      >
        play a file
      </Button>
      <Button
        onClick={async () => {
          await invoke("stop_all_sounds");
        }}
      >
        stop all sounds
      </Button>
    </div>
  );
}

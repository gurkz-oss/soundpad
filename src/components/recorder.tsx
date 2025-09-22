import { invoke } from "@tauri-apps/api/core";
import { createSignal } from "solid-js";
import { Button } from "./ui/button";
import { useQueryClient } from "@tanstack/solid-query";

export function Recorder() {
  const queryClient = useQueryClient();
  const [recording, setRecording] = createSignal(false);

  async function startRecording() {
    try {
      await invoke("start_recording");
      setRecording(true);
    } catch (err) {
      console.error(err);
    }
  }

  async function stopRecording() {
    try {
      await invoke("stop_recording");
      await queryClient.invalidateQueries({
        queryKey: ["songs", "get"],
      });
      setRecording(false);
    } catch (err) {
      console.error(err);
    }
  }

  return (
    <div class="p-4 space-y-4">
      <h2 class="text-xl font-semibold">
        System Audio Recorder (experimental!)
      </h2>

      {!recording() ? (
        <Button onClick={startRecording}>start</Button>
      ) : (
        <Button onClick={stopRecording}>stop</Button>
      )}
    </div>
  );
}

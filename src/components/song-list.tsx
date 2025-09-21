import { useQuery } from "@tanstack/solid-query";
import { invoke } from "@tauri-apps/api/core";
import { For, Show } from "solid-js/web";
import { selectedDevice } from "@/libs/device";
import { Button } from "@/components/ui/button";

export type Song = {
  name: string;
  path: string;
};

export function SongList() {
  const query = useQuery(() => ({
    queryKey: ["songs", "get"],
    queryFn: async () => await invoke<Song[]>("list_songs"),
  }));

  return (
    <Show when={query.data}>
      <div class="grid grid-cols-4 gap-2">
        <For each={query.data!}>
          {(song) => (
            <Button
              onClick={async () => {
                console.log(selectedDevice());
                if (!selectedDevice()) return alert("Select an audio device");

                await invoke("play_audio", {
                  path: song.path,
                  deviceName: selectedDevice(),
                });
              }}
            >
              {song.name}
            </Button>
          )}
        </For>
      </div>
    </Show>
  );
}

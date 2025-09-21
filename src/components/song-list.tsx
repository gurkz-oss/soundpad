import { useQuery } from "@tanstack/solid-query";
import { invoke } from "@tauri-apps/api/core";
import { For, Show } from "solid-js/web";
import { SoundpadItem } from "./soundpad-item";

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
        <For each={query.data!}>{(song) => <SoundpadItem song={song} />}</For>
      </div>
    </Show>
  );
}

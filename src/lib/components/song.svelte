<script lang="ts">
  import type { Song } from "$lib/songs.svelte";
  import { Button } from "$lib/components/ui/button";
  import { store } from "$lib/store.svelte";
  import { invoke } from "@tauri-apps/api/core";

  let { song }: { song: Song } = $props();
</script>

<Button
  onclick={async (e) => {
    e.preventDefault();
    const selectedDevice = store.state.speakerOutput;
    if (!selectedDevice) return alert("please select a device");

    await invoke("play_audio", {
      path: song.path,
      deviceName: selectedDevice,
    });
  }}>{song.name}</Button
>

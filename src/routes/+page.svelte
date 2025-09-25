<script lang="ts">
  import SongList from "$lib/components/song-list.svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { songListCtx, type Song } from "$lib/songs.svelte";
  import { resource } from "runed";
  import type { PageProps } from "./$types";
  import { devicesResourceCtx } from "$lib/devices.svelte";
  import DeviceSelector from "$lib/components/device-selector.svelte";
  import StatusButtons from "$lib/components/status-buttons.svelte";
  import SongAdder from "$lib/components/song-adder/index.js";
  import SystemAudioRecorder from "$lib/components/system-audio-recorder.svelte";

  let { data }: PageProps = $props();

  const songListResource = resource(
    () => [],
    async () => {
      return await invoke<Song[]>("list_songs");
    },
    {
      initialValue: data.initialSongs,
    }
  );

  const devicesResource = resource(
    () => [],
    async () => {
      return await invoke<string[]>("list_audio_devices");
    },
    {
      initialValue: data.initialDevices,
    }
  );

  devicesResourceCtx.set(devicesResource);
  songListCtx.set(songListResource);
</script>

<p>hi, home page will soon be finished</p>

<SongAdder />
<DeviceSelector />
<SongList />
<br />
<SystemAudioRecorder />
<br />
<StatusButtons />
<br />

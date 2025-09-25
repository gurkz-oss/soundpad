<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import Button from "./ui/button/button.svelte";
  import { songListCtx } from "$lib/songs.svelte";
  let status = $state<"idle" | "recording">("idle");
  let isRecording = $derived(status === "recording");

  const songListResource = songListCtx.get();

  async function startRecording() {
    try {
      await invoke("start_recording");
      status = "recording";
    } catch (err) {
      console.error(err);
    }
  }

  async function stopRecording() {
    try {
      await invoke("stop_recording");
      await songListResource.refetch();
      status = "idle";
    } catch (err) {
      console.error(err);
    }
  }
</script>

<div class="p-4 space-y-4">
  <h2 class="text-xl font-semibold">System Audio Recorder (experimental!)</h2>

  {#if !isRecording}
    <Button onclick={startRecording}>start</Button>
  {:else}
    <Button onclick={stopRecording}>stop</Button>
  {/if}
</div>

<script lang="ts">
  import { openPath } from "@tauri-apps/plugin-opener";
  import { appDataDir } from "@tauri-apps/api/path";
  import SquareArrowOutUpRight from "@lucide/svelte/icons/square-arrow-out-up-right";
  import { open } from "@tauri-apps/plugin-dialog";
  import { warn } from "@tauri-apps/plugin-log";
  import { Button } from "$lib/components/ui/button";
  import { songListCtx } from "$lib/songs.svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { store } from "$lib/store.svelte";

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

    return path;
  }

  async function playFile(path: string | null) {
    if (!path) return warn("no path got provided");

    await invoke("play_audio", {
      path: path,
      deviceName: store.state.speakerOutput,
    });
  }

  const songListResource = songListCtx.get();
</script>

<div class="flex flex-row gap-2">
  <Button
    onclick={async () => {
      openPath(await appDataDir());
    }}
    class="flex flex-row gap-2"
  >
    <SquareArrowOutUpRight size={16} />
    open soundboard folder
  </Button>
  <Button
    onclick={async () => {
      const file = await selectFile();
      await playFile(file);
    }}
  >
    play a file
  </Button>
  <Button onclick={() => songListResource.refetch()}>refresh song list</Button>
</div>

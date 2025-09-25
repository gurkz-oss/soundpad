<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { open } from "@tauri-apps/plugin-dialog";
  import { warn } from "@tauri-apps/plugin-log";
  import { Button } from "../ui/button";
  import { songListCtx } from "$lib/songs.svelte";

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

  async function addFile(path: string | null) {
    if (!path) return warn("no path got provided");

    await invoke("add_song", {
      path: path,
    });
  }

  let {
    closeSongAdderDialog,
  }: {
    closeSongAdderDialog: () => void;
  } = $props();

  const songListResource = songListCtx.get();
</script>

<Button
  class="w-fit"
  onclick={async () => {
    const file = await selectFile();
    await addFile(file);
    await songListResource.refetch();
    closeSongAdderDialog();
  }}
>
  add file
</Button>

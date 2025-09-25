<script lang="ts">
  import { songListCtx, type Song } from "$lib/songs.svelte";
  import { Button } from "../ui/button";
  import {
    DialogHeader,
    DialogTitle,
    DialogDescription,
    Close,
    DialogFooter,
  } from "../ui/dialog";
  import * as path from "@tauri-apps/api/path";
  import { BaseDirectory, remove } from "@tauri-apps/plugin-fs";

  let { closeDialog, song }: { closeDialog: () => void; song: Song } = $props();
  const songListResource = songListCtx.get();
</script>

<DialogHeader>
  <DialogTitle>delete "{song.name}"?</DialogTitle>
  <DialogDescription>
    this will permanently delete this file, and it cannot be recovered.
  </DialogDescription>
</DialogHeader>
<DialogFooter>
  <Button
    onclick={async () =>
      await remove(await path.join("songs", `${song.name}.mp3`), {
        baseDir: BaseDirectory.AppData,
      })
        .then(async () => await songListResource.refetch())
        .then(() => closeDialog())}
    variant={"destructive"}
  >
    yes
  </Button>
  <Close>
    <Button variant={"outline"}>no</Button>
  </Close>
</DialogFooter>

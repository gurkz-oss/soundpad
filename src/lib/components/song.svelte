<script lang="ts">
  import type { Song } from "$lib/songs.svelte";
  import { buttonVariants } from "$lib/components/ui/button";
  import { store } from "$lib/store.svelte";
  import {
    ContextMenu,
    ContextMenuContent,
    ContextMenuItem,
    ContextMenuSeparator,
    ContextMenuTrigger,
  } from "$lib/components/ui/context-menu/index.js";
  import { invoke } from "@tauri-apps/api/core";
  import { cn } from "$lib/utils";
  import { Dialog, DialogTrigger } from "./ui/dialog";
  import DialogContent from "./ui/dialog/dialog-content.svelte";
  import Delete from "./song-edit/delete.svelte";
  import Rename from "./song-edit/rename.svelte";
  import { revealItemInDir } from "@tauri-apps/plugin-opener";
  import * as path from "@tauri-apps/api/path";

  let { song }: { song: Song } = $props();
  let open = $state(false);
  let dialogMode = $state<"delete" | "rename">("delete");

  function closeDialog() {
    open = false;
  }
</script>

<Dialog bind:open>
  <ContextMenu>
    <ContextMenuTrigger
      class={cn(
        buttonVariants({ variant: "default" }),
        "select-none whitespace-nowrap overflow-hidden text-ellipsis truncate w-full text-left"
      )}
      onclick={async (e) => {
        e.preventDefault();
        const selectedDevice = store.state.speakerOutput;
        if (!selectedDevice) return alert("please select a device");

        await invoke("play_audio", {
          path: song.path,
          deviceName: selectedDevice,
        });
      }}>{song.name}</ContextMenuTrigger
    >
    <ContextMenuContent>
      <DialogTrigger class="w-full" onclick={() => (dialogMode = "rename")}>
        <ContextMenuItem>rename</ContextMenuItem>
      </DialogTrigger>

      <DialogTrigger class="w-full" onclick={() => (dialogMode = "delete")}>
        <ContextMenuItem>delete</ContextMenuItem>
      </DialogTrigger>
      <ContextMenuSeparator />

      <ContextMenuItem
        class="w-full"
        onclick={async () =>
          revealItemInDir(
            await path.join(
              await path.appDataDir(),
              "songs",
              `${song.name}.mp3`
            )
          )}>reveal in explorer</ContextMenuItem
      >
    </ContextMenuContent>
  </ContextMenu>

  <DialogContent>
    {#if dialogMode === "delete"}
      <Delete {closeDialog} {song} />
    {:else if dialogMode === "rename"}
      <Rename {closeDialog} {song} />
    {/if}
  </DialogContent>
</Dialog>

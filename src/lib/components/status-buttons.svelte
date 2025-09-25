<script lang="ts">
  import { songListCtx } from "$lib/songs.svelte";
  import Button from "$lib/components/ui/button/button.svelte";
  import { openPath } from "@tauri-apps/plugin-opener";
  import { appDataDir } from "@tauri-apps/api/path";
  import SquareArrowOutUpRight from "@lucide/svelte/icons/square-arrow-out-up-right";

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
  <Button onclick={() => songListResource.refetch()}>refresh song list</Button>
</div>

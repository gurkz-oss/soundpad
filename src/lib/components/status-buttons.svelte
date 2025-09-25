<script lang="ts">
  import { songListCtx } from "$lib/songs.svelte";
  import Button from "$lib/components/ui/button/button.svelte";
  const songListResource = songListCtx.get();
</script>

<div class="flex flex-row gap-2">
  <Button onclick={() => songListResource.refetch()}>refresh song list</Button>
</div>

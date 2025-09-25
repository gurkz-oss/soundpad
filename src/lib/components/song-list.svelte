<script lang="ts">
  import Song from "./song.svelte";
  import { songListCtx } from "$lib/songs.svelte";
  const songListResource = songListCtx.get();
</script>

{#if songListResource.current}
  <div class="grid grid-cols-4 gap-2">
    {#each songListResource.current ?? [] as song}
      <Song {song} />
    {/each}
  </div>
{/if}

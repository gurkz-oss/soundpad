<script lang="ts">
  import Song from "./song.svelte";
  import { songListCtx } from "$lib/songs.svelte";
  const songListResource = songListCtx.get();
</script>

{#if songListResource.current}
  <div
    class="grid grid-cols-2 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 gap-2 overflow-hidden"
  >
    {#each songListResource.current ?? [] as song}
      <Song {song} />
    {/each}
  </div>
{/if}

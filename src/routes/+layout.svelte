<script lang="ts">
  import Header from "$lib/components/header.svelte";
  import { checkForAppUpdates } from "$lib/update";
  import "../app.css";
  import { ModeWatcher } from "mode-watcher";

  let { children } = $props();

  $effect(() => {
    checkForAppUpdates(false);
  });
</script>

<ModeWatcher />
<Header />
<main class="p-2">{@render children()}</main>

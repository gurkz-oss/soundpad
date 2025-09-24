<script lang="ts">
  import { openUrl } from "@tauri-apps/plugin-opener";
  import GitHub from "@lucide/svelte/icons/github";
  import Button from "$lib/components/ui/button/button.svelte";
  import {
    Table,
    TableBody,
    TableCell,
    TableRow,
  } from "$lib/components/ui/table/index";
  import type { PageProps } from "./$types";

  let { data }: PageProps = $props();
</script>

<h1 class="text-3xl">about</h1>

<div class="pb-2">
  <h2 class="text-2xl">Soundpad {data.appVersion} by Gurkan</h2>
  <p>this software is licensed under the GNU GPL 3.0 license</p>
  <Button
    onclick={async () => {
      await openUrl("https://github.com/gurkz-oss/soundpad");
    }}
    variant={"outline"}
  >
    <GitHub /> GitHub
  </Button>
</div>

<Table class="w-fit">
  <TableBody>
    <TableRow>
      <TableCell>Tauri Version</TableCell>
      <TableCell>{data.tauriVersion}</TableCell>
    </TableRow>
    <TableRow>
      <TableCell>App Version</TableCell>
      <TableCell>{data.appVersion}</TableCell>
    </TableRow>
  </TableBody>
</Table>

<p>
  this app uses
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <!-- svelte-ignore a11y_missing_attribute -->
  <a
    class="cursor-pointer text-blue-500 hover:text-blue-700 underline hover:no-underline transition-all duration-300 ease-in-out"
    onclick={() => openUrl("https://github.com/abdipr/myinstants-api")}
  >
    https://github.com/abdipr/myinstants-api
  </a>
  to download files from MyInstants, licensed under the MIT license
</p>

<p>
  this app uses
  <!-- svelte-ignore a11y_click_events_have_key_events -->
  <!-- svelte-ignore a11y_no_static_element_interactions -->
  <!-- svelte-ignore a11y_missing_attribute -->
  <a
    class="cursor-pointer text-blue-500 hover:text-blue-700 underline hover:no-underline transition-all duration-300 ease-in-out"
    onclick={() => openUrl("https://www.mp3dev.org")}
  >
    libmp3lame.dll
  </a>{" "}
  for recording system audio.
</p>

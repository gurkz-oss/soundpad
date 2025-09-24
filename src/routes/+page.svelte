<script lang="ts">
  import { store } from "$lib/store.svelte";
  import {
    Select,
    SelectContent,
    SelectGroup,
    SelectItem,
    SelectLabel,
    SelectTrigger,
  } from "$lib/components/ui/select";
  import type { PageProps } from "./$types";

  let { data }: PageProps = $props();
</script>

<p>hi, home page will soon be finished</p>

{#if data.deviceList.length > 0}
  <Select
    type={"single"}
    name="speaker_output"
    required
    bind:value={store.state.speakerOutput}
  >
    <SelectTrigger>
      {store.state.speakerOutput}
    </SelectTrigger>

    <SelectContent>
      <SelectGroup>
        <SelectLabel>devices</SelectLabel>
        {#each data.deviceList as device (device)}
          <SelectItem value={device} label={device}>
            {device}
          </SelectItem>
        {/each}
      </SelectGroup>
    </SelectContent>
  </Select>
{/if}

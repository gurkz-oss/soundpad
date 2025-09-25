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
  import { devicesResourceCtx } from "$lib/devices.svelte";

  const devicesResource = devicesResourceCtx.get();
</script>

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
      {#each devicesResource.current ?? [] as device (device)}
        <SelectItem value={device} label={device}>
          {device}
        </SelectItem>
      {/each}
    </SelectGroup>
  </SelectContent>
</Select>

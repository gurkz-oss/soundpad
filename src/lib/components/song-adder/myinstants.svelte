<script module lang="ts">
  import * as v from "valibot";

  // in case i need to use this sometime
  export const addFromMyInstantsSchema = v.strictObject({
    url: v.pipe(
      v.string(),
      v.nonEmpty("please enter a url"),
      v.url("the url is badly formatted"),
      v.check((val) => {
        try {
          const { hostname } = new URL(val);
          return (
            hostname === "myinstants.com" || hostname === "www.myinstants.com"
          );
        } catch (e) {
          return false;
        }
      }, "url must be from myinstants.com")
    ),
  });
</script>

<script lang="ts">
  import { createForm, Field, Form } from "@formisch/svelte";
  import { Input } from "$lib/components/ui/input/index.js";
  import { invoke } from "@tauri-apps/api/core";
  import { songListCtx } from "$lib/songs.svelte";
  import { Button } from "../ui/button";
  const form = createForm({
    schema: addFromMyInstantsSchema,
  });

  let {
    closeSongAdderDialog,
  }: {
    closeSongAdderDialog: () => void;
  } = $props();

  const songListResource = songListCtx.get();
</script>

<Form
  of={form}
  onsubmit={async ({ url }) => {
    await invoke("download_from_myinstants", {
      url: url,
    }).then(async () => {
      await songListResource.refetch();
      closeSongAdderDialog();
    });
  }}
>
  <div>
    <Field of={form} path={["url"]}>
      {#snippet children(field)}
        <div class="flex w-full max-w-sm items-center space-x-2">
          <Input
            aria-invalid={field.isValid ? "false" : "true"}
            {...field.props}
          />
          <Button disabled={form.isSubmitting} type="submit">add</Button>
        </div>
        {#if !field.isValid}
          <span class="text-destructive text-sm font-medium">
            {field.errors && field.errors[0]}
          </span>
        {/if}
      {/snippet}
    </Field>
  </div>
</Form>

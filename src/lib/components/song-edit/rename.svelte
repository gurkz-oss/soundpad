<script lang="ts">
  import { songListCtx, type Song } from "$lib/songs.svelte";
  import { Button } from "../ui/button";
  import { DialogHeader, DialogTitle } from "../ui/dialog";
  import * as path from "@tauri-apps/api/path";
  import * as v from "valibot";
  import { BaseDirectory, rename } from "@tauri-apps/plugin-fs";
  import { createForm, Field, Form } from "@formisch/svelte";
  import { Input } from "../ui/input";
  import { createRegExp, char, charIn, exactly } from "magic-regexp";

  const forbiddenCharsRegex = createRegExp(
    exactly("").notBefore(char.times.any(), charIn('<>:"/|?*')).at.lineStart(),
    char.times.any().at.lineEnd()
  ); // Negative lookahead for forbidden characters
  const forbiddenExtensionRegex = createRegExp(
    exactly("").notBefore(char.times.any(), charIn('<>:"/|?*')).at.lineStart(),
    char.times.any().at.lineEnd()
  ); // Negative lookahead for file extensions

  const renameFormSchema = v.strictObject({
    name: v.pipe(
      v.string(),
      v.nonEmpty("please provide a name"),
      v.regex(forbiddenCharsRegex, "name contains forbidden characters"),
      v.regex(forbiddenExtensionRegex, "name cannot contain a file extension")
    ),
  });

  let { closeDialog, song }: { closeDialog: () => void; song: Song } = $props();
  const songListResource = songListCtx.get();

  const form = createForm({
    schema: renameFormSchema,
    initialInput: {
      name: song.name,
    },
  });
</script>

<DialogHeader>
  <DialogTitle>rename "{song.name}"?</DialogTitle>
</DialogHeader>
<Form
  of={form}
  onsubmit={async (output) =>
    await rename(
      await path.join("songs", `${song.name}.mp3`),
      await path.join("songs", `${output.name}.mp3`),
      {
        oldPathBaseDir: BaseDirectory.AppData,
        newPathBaseDir: BaseDirectory.AppData,
      }
    ).then(async () => {
      await songListResource.refetch();
      closeDialog();
    })}
>
  <Field of={form} path={["name"]}>
    {#snippet children(field)}
      <div class="flex w-full max-w-sm items-center space-x-2">
        <Input
          defaultValue={song.name}
          aria-invalid={field.isValid ? "false" : "true"}
          {...field.props}
        />
        <Button disabled={form.isSubmitting} type="submit">rename</Button>
      </div>
      {#if !field.isValid}
        <span class="text-destructive text-sm font-medium">
          {field.errors && field.errors[0]}
        </span>
      {/if}
    {/snippet}
  </Field>
</Form>

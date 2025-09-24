import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from "@/components/ui/dialog";
import { DialogTriggerProps } from "@kobalte/core/dialog";
import { createForm, Field, Form } from "@formisch/solid";
import {
  TextField,
  TextFieldErrorMessage,
  TextFieldRoot,
} from "@/components/ui/textfield";

import { Button } from "./ui/button";
import * as v from "valibot";
import { FileAdder } from "./file-adder";
import { createSignal } from "solid-js";
import { useMutation } from "@tanstack/solid-query";
import { invoke } from "@tauri-apps/api/core";

const addFromMyInstantsSchema = v.strictObject({
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

function AddFromMyInstants() {
  const form = createForm({
    schema: addFromMyInstantsSchema,
  });
  const mutation = useMutation(() => ({
    mutationKey: ["song", "add", "myinstants"],
    mutationFn: async ({ url }: { url: string }, ctx) => {
      await invoke("download_from_myinstants", {
        url: url,
      }).then(async () => {
        await ctx.client.invalidateQueries({
          queryKey: ["songs", "get"],
        });
      });
    },
    onSuccess: () => {
      closeSongAdderDialog();
    },
  }));

  return (
    <Form
      of={form}
      onSubmit={async (output) => mutation.mutateAsync({ url: output.url })}
    >
      <Field of={form} path={["url"]}>
        {(field) => (
          <div>
            <TextFieldRoot
              validationState={field.isValid ? "valid" : "invalid"}
            >
              <TextField
                {...field.props}
                type="url"
                placeholder="myinstants url"
              />
              <TextFieldErrorMessage>
                {field.errors && field.errors[0]}
              </TextFieldErrorMessage>
            </TextFieldRoot>
          </div>
        )}
      </Field>
      <Button disabled={form.isSubmitting} type="submit">
        add
      </Button>
    </Form>
  );
}

const [dialogOpen, setDialogOpen] = createSignal(false);

export function closeSongAdderDialog() {
  setDialogOpen(false);
}

export function SongAdder() {
  return (
    <Dialog open={dialogOpen()} onOpenChange={setDialogOpen}>
      <DialogTrigger
        as={(props: DialogTriggerProps) => <Button {...props}>add song</Button>}
      />
      <DialogContent>
        <DialogHeader>
          <DialogTitle>what song would you like to add?</DialogTitle>
          <DialogDescription>add from myinstants</DialogDescription>
          <AddFromMyInstants />

          <DialogDescription>add from a file</DialogDescription>
          <FileAdder />
        </DialogHeader>
      </DialogContent>
    </Dialog>
  );
}

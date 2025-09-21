import { SoundItem } from "@/schemas/sound";
import { Button } from "@/components/ui/button";
import { selectedDevice } from "@/libs/device";
import { invoke } from "@tauri-apps/api/core";
import {
  ContextMenu,
  ContextMenuContent,
  ContextMenuItem,
  ContextMenuTrigger,
} from "@/components/ui/context-menu";
import { ContextMenuTriggerProps } from "@kobalte/core/context-menu";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from "@/components/ui/dialog";
import { createSignal, Match, Switch } from "solid-js";
import * as v from "valibot";
import { createForm, Field, Form } from "@formisch/solid";
import {
  TextField,
  TextFieldErrorMessage,
  TextFieldRoot,
} from "./ui/textfield";
import { BaseDirectory, rename } from "@tauri-apps/plugin-fs";
import { useQueryClient } from "@tanstack/solid-query";
import * as path from "@tauri-apps/api/path";

const [dialogMode, setDialogMode] = createSignal<"rename">("rename");

const forbiddenCharsRegex = new RegExp(`^(?!.*[<>:"/\\|?*]).*$`); // Negative lookahead for forbidden characters
const forbiddenExtensionRegex = new RegExp(`^(?!.*\\.[^\\\\]*$).*$`); // Negative lookahead for file extensions

const renameFormSchema = v.strictObject({
  name: v.pipe(
    v.string(),
    v.nonEmpty("please provide a name"),
    v.regex(forbiddenCharsRegex, "name contains forbidden characters"),
    v.regex(forbiddenExtensionRegex, "name cannot contain a file extension")
  ),
});

function Rename(props: { song: SoundItem }) {
  const form = createForm({
    schema: renameFormSchema,
    initialInput: {
      name: props.song.name,
    },
  });
  const queryClient = useQueryClient();

  return (
    <>
      <DialogHeader>
        <DialogTitle>
          rename "{props.song.name} - {props.song.path}"
        </DialogTitle>
        <Form
          of={form}
          onSubmit={async (output) =>
            await rename(
              await path.join("songs", `${props.song.name}.mp3`),
              await path.join("songs", `${output.name}.mp3`),
              {
                oldPathBaseDir: BaseDirectory.AppData,
                newPathBaseDir: BaseDirectory.AppData,
              }
            ).then(
              async () =>
                await queryClient.invalidateQueries({
                  queryKey: ["songs", "get"],
                })
            )
          }
        >
          <Field of={form} path={["name"]}>
            {(field) => (
              <div>
                <TextFieldRoot
                  defaultValue={props.song.name}
                  validationState={field.isValid ? "valid" : "invalid"}
                >
                  <TextField {...field.props} />
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
      </DialogHeader>
    </>
  );
}

export function SoundpadItem(props: { song: SoundItem }) {
  return (
    <Dialog>
      <ContextMenu>
        <ContextMenuTrigger
          as={(ctxMenuProps: ContextMenuTriggerProps) => (
            <>
              <Button
                {...ctxMenuProps}
                onClick={async () => {
                  console.log(selectedDevice());
                  if (!selectedDevice()) return alert("Select an audio device");

                  await invoke("play_audio", {
                    path: props.song.path,
                    deviceName: selectedDevice(),
                  });
                }}
              >
                {props.song.name}
              </Button>
            </>
          )}
        />
        <ContextMenuContent>
          <ContextMenuItem>
            <DialogTrigger onClick={() => setDialogMode("rename")}>
              rename
            </DialogTrigger>
          </ContextMenuItem>
        </ContextMenuContent>
      </ContextMenu>

      <DialogContent>
        <Switch>
          <Match when={dialogMode() === "rename"}>
            <Rename song={props.song} />
          </Match>
        </Switch>
      </DialogContent>
    </Dialog>
  );
}

import { open } from "@tauri-apps/plugin-dialog";
import { invoke } from "@tauri-apps/api/core";
import { useQueryClient } from "@tanstack/solid-query";
import { Button } from "./ui/button";

async function selectFile() {
  const path = await open({
    multiple: false,
    directory: false,
    filters: [
      {
        name: "mp3 files",
        extensions: ["mp3"],
      },
    ],
  });

  await invoke("add_song", {
    path: path,
  });
}

export function FileAdder() {
  const queryClient = useQueryClient();
  return (
    <Button
      onClick={async () => {
        await selectFile();
        await queryClient.invalidateQueries({
          queryKey: ["songs", "get"],
        });
      }}
    >
      add song
    </Button>
  );
}

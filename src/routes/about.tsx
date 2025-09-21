import { getTauriVersion, getVersion } from "@tauri-apps/api/app";
import { createResource } from "solid-js";
import { Table, TableBody, TableCell, TableRow } from "@/components/ui/table";
import { Button } from "@/components/ui/button";
import GitHub from "lucide-solid/icons/github";
import { openUrl } from "@tauri-apps/plugin-opener";

export function About() {
  const [appVersion] = createResource(() => getVersion());
  const [tauriVersion] = createResource(() => getTauriVersion());
  return (
    <>
      <h1 class="text-3xl">about</h1>

      <div class="pb-2">
        <h2 class="text-2xl">Soundpad {appVersion()} by Gurkan</h2>
        <p>this software is licensed under the GNU GPL 3.0 license</p>
        <Button
          onClick={async () => {
            await openUrl("https://github.com/gurkz-oss/soundpad");
          }}
          variant={"outline"}
        >
          <GitHub class="pr-2" /> GitHub
        </Button>
      </div>

      <Table class="w-fit">
        <TableBody>
          <TableRow>
            <TableCell>Tauri Version</TableCell>
            <TableCell>{tauriVersion()}</TableCell>
          </TableRow>
          <TableRow>
            <TableCell>App Version</TableCell>
            <TableCell>{appVersion()}</TableCell>
          </TableRow>
        </TableBody>
      </Table>

      <p>
        this app uses{" "}
        <a
          class="cursor-pointer text-blue-500 hover:text-blue-700 underline hover:no-underline transition-all duration-300 ease-in-out"
          onClick={() => openUrl("https://github.com/abdipr/myinstants-api")}
        >
          https://github.com/abdipr/myinstants-api
        </a>{" "}
        to download files from MyInstants, licensed under the MIT license
      </p>
    </>
  );
}

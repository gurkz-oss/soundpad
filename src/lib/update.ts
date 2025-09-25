import { check } from "@tauri-apps/plugin-updater";
import { ask, message } from "@tauri-apps/plugin-dialog";
import { relaunch } from "@tauri-apps/plugin-process";

export async function checkForAppUpdates(onUserClick: boolean) {
  const update = await check();
  if (update) {
    const yes = await ask(
      `Update to ${update.version} is available!\n\nRelease notes: ${update.body}`,
      {
        title: "Update Available",
        kind: "info",
        okLabel: "Update",
        cancelLabel: "Cancel",
      }
    );
    if (yes) {
      await update.downloadAndInstall();
      await relaunch();
    }
  } else if (onUserClick) {
    await message("you are on the latest version!", {
      title: "No Update Available",
      kind: "info",
      okLabel: "OK",
    });
  }
}

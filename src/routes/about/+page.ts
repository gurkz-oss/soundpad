import type { PageLoad } from "./$types";
import { getTauriVersion, getVersion } from "@tauri-apps/api/app";

export const load: PageLoad = async () => {
  return {
    appVersion: await getVersion(),
    tauriVersion: await getTauriVersion(),
  };
};

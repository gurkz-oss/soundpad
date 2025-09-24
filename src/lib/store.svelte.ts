import { RuneStore } from "@tauri-store/svelte";

export type Store = {
  speakerOutput: string;
};

export const store = new RuneStore<Store>(
  "soundboard_store",
  {
    speakerOutput: "",
  },
  {
    autoStart: true,
    saveOnChange: true,
    saveStrategy: "debounce",
    saveInterval: 1000,
  }
);

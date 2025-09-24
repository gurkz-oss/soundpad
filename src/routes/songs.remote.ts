import { query } from "$app/server";
import type { Song } from "$lib/sound";

export const getSongs = query(async () => {
  // TODO: make this work
  return [] as Song[];
});

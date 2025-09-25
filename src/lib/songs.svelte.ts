import * as v from "valibot";
import { Context, type ResourceReturn } from "runed";

const Song = v.object({
  name: v.string(),
  path: v.string(),
});

type Song = v.InferOutput<typeof Song>;

const songListCtx = new Context<
  ResourceReturn<
    {
      name: string;
      path: string;
    }[],
    unknown,
    false
  >
>("song_list_ctx");

export { Song, songListCtx };

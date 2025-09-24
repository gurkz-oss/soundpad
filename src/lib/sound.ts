import * as v from "valibot";

const Song = v.object({
  name: v.string(),
  path: v.string(),
});

type Song = v.InferOutput<typeof Song>;

export { Song };

import * as v from "valibot";

const SoundItem = v.object({
  name: v.string(),
  path: v.string(),
});

type SoundItem = v.InferOutput<typeof SoundItem>;

export { SoundItem };

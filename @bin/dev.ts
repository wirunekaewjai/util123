import { $ } from "bun";
import { rm } from "node:fs/promises";

await rm(".cache", {
  force: true,
  recursive: true,
});

await rm("assets", {
  force: true,
  recursive: true,
});

const css = "tailwindcss -i ./tailwind.css -o ./.cache/assets/style.css";
const ts = "bun build ./src/client/app.ts --outdir ./.cache/assets --splitting --chunk-naming chunk-[hash].[ext]";
const map = "bun ./@bin/build-assets.ts";
const rs = "cargo run";

await $`cargo watch -s '${css} && ${ts} && ${map} && ${rs}'`;
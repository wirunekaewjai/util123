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

const tsEntryPoints = [
  "./src/client/app.ts",
  "./src/client/auto-reload.ts",
];

const style = "tailwindcss -i ./tailwind.css -o ./.cache/assets/style.css";
const scripts = `bun build ${tsEntryPoints.join(" ")} --outdir ./.cache/assets --splitting --chunk-naming chunk-[hash].[ext]`;
const assets = "bun ./@bin/build-assets.ts";
const rs = "cargo run";

await $`cargo watch -s '${style} && ${scripts} && ${assets} && ${rs}'`;
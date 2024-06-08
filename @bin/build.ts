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

await $`tailwindcss -i ./tailwind.css -o ./.cache/assets/style.css --minify`;
await $`bun build ./src/client/app.ts --outdir ./.cache/assets --minify --splitting --chunk-naming chunk-[hash].[ext]`;
await $`bun ./@bin/build-assets.ts --minify`;
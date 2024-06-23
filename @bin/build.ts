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
];

await $`tailwindcss -i ./tailwind.css -o ./.cache/assets/style.css --minify`;

for (const entryPoint of tsEntryPoints) {
  await $`bun build ${entryPoint} --outdir ./.cache/assets --minify --splitting --chunk-naming chunk-[hash].[ext]`;
}

await $`bun ./@bin/build-assets.ts --minify`;
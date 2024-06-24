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

await $`bun ./@bin/build-styles.ts --minify`;
await $`bun ./@bin/build-scripts.ts --minify`;
await $`bun ./@bin/build-assets.ts --minify`;
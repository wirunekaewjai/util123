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

const scripts = [
  "bun ./@bin/build-styles.ts",
  "bun ./@bin/build-scripts.ts",
  "bun ./@bin/build-assets.ts",
  "cargo run",
];

await $`cargo watch -s '${scripts.join(" && ")}'`;
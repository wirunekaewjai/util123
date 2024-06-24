import { $ } from "bun";

const minify = process.argv.includes("--minify");

if (minify) {
  await $`tailwindcss -i ./tailwind.css -o ./.cache/assets/style.css --minify`;
} else {
  await $`tailwindcss -i ./tailwind.css -o ./.cache/assets/style.css`;
}
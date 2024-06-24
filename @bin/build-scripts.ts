import path from "node:path";
import { styleText } from "node:util";

console.log();

const minify = process.argv.includes("--minify");
const entrypoints = minify ? [
  "./src/client/app-admin.ts",
  "./src/client/app.ts",
] : [
  "./src/client/app-admin.ts",
  "./src/client/app.ts",
  "./src/client/auto-reload.ts",
];

const items: [string, string][] = [];

let padding1 = 0;
let padding2 = 0;

for (const entrypoint of entrypoints) {
  const output = await Bun.build({
    entrypoints: [entrypoint],
    outdir: "./.cache/assets",
    minify,
    splitting: true,
    naming: {
      chunk: "chunk-[hash].[ext]",
    },
  });

  for (const artifact of output.outputs) {
    const pathObject = path.parse(artifact.path);
    const name = pathObject.base.startsWith("chunk-") ? ` - ${pathObject.base}` : pathObject.base;
    const size = (artifact.size / 1024).toFixed(2);

    items.push([name, size]);

    if (name.length > padding1) {
      padding1 = name.length;
    }

    if (size.length > padding2) {
      padding2 = size.length;
    }
  }

  if (output.outputs.length > 1) {
    items.push(["", ""]);
  }
}

for (const [name, size] of items) {
  if (!name) {
    console.log();
    continue;
  }

  const text = `${name.padEnd(padding1, " ")}  ${size.padStart(padding2, " ")} KB`;

  if (name.startsWith(" ")) {
    console.log(styleText("blueBright", text));
  } else {
    console.log(styleText("greenBright", text));
  }
}

console.log();
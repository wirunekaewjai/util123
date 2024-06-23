import { Glob } from "bun";
import path from "node:path";

const minify = process.argv.includes("--minify");

const assetsGlob = new Glob("./.cache/assets/**/*").scanSync({
  dot: true,
  onlyFiles: true,
});

// const publicGlob = new Glob("./public/**/*").scanSync({
//   onlyFiles: true,
// });

const map: Record<string, string> = {};

for (const file of assetsGlob) {
  const data = await Bun.file(file).arrayBuffer();
  const hash = Bun.hash.wyhash(data).toString(16);

  // map["/" + file.split(".cache/")[1]] = hash;

  const filePath = file.split(".cache/")[1];
  const pathObject = path.parse(filePath);

  const base = pathObject.name;
  const ext = pathObject.ext;
  const name = `${base}.${hash}${ext}`;

  const routePath = `/${pathObject.dir}/${name}`.replaceAll("//", "/");
  const routeKey = `/${filePath}`;

  map[routePath] = file;
  map[routeKey] = routePath;
}

// for (const file of publicGlob) {
//   const data = await Bun.file(file).arrayBuffer();
//   const hash = Bun.hash.wyhash(data).toString(16);

//   // map["/" + file.split("public/")[1]] = hash;

//   const filePath = file.split("public/")[1];
//   const pathObject = path.parse(filePath);

//   const base = pathObject.name;
//   const ext = pathObject.ext;
//   const name = `${base}.${hash}${ext}`;

//   const routePath = `/${pathObject.dir}/${name}`.replaceAll("//", "/");
//   const routeKey = `/${filePath}`;

//   map[routePath] = file;
//   map[routeKey] = routePath;
// }

const text = minify ? JSON.stringify(map) : JSON.stringify(map, null, 2);
Bun.write(Bun.file(".cache/map.json"), text);
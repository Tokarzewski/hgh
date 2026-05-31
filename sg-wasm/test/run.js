// Node harness: feed the decoded image to the WASM pipeline, write a GLB.
const fs = require("fs");
const path = require("path");
const wasm = require("../pkg-node/sg_wasm.js");

const dir = __dirname;
const meta = JSON.parse(fs.readFileSync(path.join(dir, "img.json")));
const px = new Uint8Array(fs.readFileSync(path.join(dir, "img.rgba")));

const cfg = ["diamond", 200, 300, 16, 8, 1.2, 4, 8, 0.55, 42]; // matches Python CFG defaults

console.log("tiles:", wasm.count_tiles(cfg[0], cfg[1], cfg[2], cfg[3]));

// time it (a few runs)
let glb;
for (let i = 0; i < 3; i++) {
  const t0 = process.hrtime.bigint();
  glb = wasm.generate_glb(px, meta.w, meta.h, meta.ch, ...cfg);
  const t1 = process.hrtime.bigint();
  console.log(`run ${i}: ${(Number(t1 - t0) / 1e6).toFixed(1)} ms  -> ${glb.length} bytes`);
}
fs.writeFileSync(path.join(dir, "stained_glass_rust.glb"), Buffer.from(glb));

// also emit a couple other patterns to prove parity
for (const pat of ["hexagonal", "triangle", "brick"]) {
  const g = wasm.generate_glb(px, meta.w, meta.h, meta.ch, pat, 200, 300, 16, 8, 1.2, 4, 8, 0.55, 42);
  fs.writeFileSync(path.join(dir, `sg_${pat}.glb`), Buffer.from(g));
  console.log(`${pat}: ${wasm.count_tiles(pat, 200, 300, 16)} tiles, ${g.length} bytes`);
}
console.log("wrote GLBs");

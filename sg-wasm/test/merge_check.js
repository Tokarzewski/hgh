// Quick check of cell merging (issue #9): on a half-red / half-blue image,
// merging should remove the lead lines inside each constant-colour region,
// leaving lead only on the panel border and along the colour boundary.
const wasm = require("../pkg-node/sg_wasm.js");

const W = 64, H = 64;
const px = new Uint8Array(W * H * 4);
for (let y = 0; y < H; y++)
  for (let x = 0; x < W; x++) {
    const i = (y * W + x) * 4;
    if (x < W / 2) { px[i] = 220; px[i + 1] = 30; px[i + 2] = 30; }
    else { px[i] = 30; px[i + 1] = 60; px[i + 2] = 220; }
    px[i + 3] = 255;
  }
const pal = new Uint8Array([220, 30, 30, 30, 60, 220]);

const leadCount = (buf) => {
  let n = 0;
  for (let i = 0; i < buf.length; i += 4)
    if (buf[i] === 28 && buf[i + 1] === 28 && buf[i + 2] === 32) n++;
  return n;
};

for (const pat of ["rectangle", "brick", "diamond", "hexagonal", "triangle"]) {
  const a = wasm.render_preview(px, W, H, 4, pat, 100, 100, 20, 2, 1.0, pal, 200, 200, false);
  const b = wasm.render_preview(px, W, H, 4, pat, 100, 100, 20, 2, 1.0, pal, 200, 200, true);
  const ga = wasm.render_glb(px, W, H, 4, pat, 100, 100, 20, 2, 4, 8, 0.55, pal, false);
  const gb = wasm.render_glb(px, W, H, 4, pat, 100, 100, 20, 2, 4, 8, 0.55, pal, true);
  console.log(
    `${pat.padEnd(10)} lead px: ${leadCount(a)} -> ${leadCount(b)}  glb bytes: ${ga.length} -> ${gb.length}`
  );
  if (leadCount(b) >= leadCount(a)) throw new Error(pat + ": merging removed no lead");
  if (gb.length >= ga.length) throw new Error(pat + ": merged GLB not smaller");
  // GLB magic + version sanity
  for (const g of [ga, gb]) {
    const dv = new DataView(g.buffer, g.byteOffset, g.byteLength);
    if (dv.getUint32(0, true) !== 0x46546c67) throw new Error("bad GLB magic");
  }
}
console.log("merge check OK");

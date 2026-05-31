# sg-wasm — stained-glass pipeline in Rust/WASM

A port of the Python notebook pipeline (`pipeline/stained_glass_hatch.ipynb`) to a
dependency-light Rust crate compiled to WebAssembly. Runs the whole thing in the browser:

```
image pixels → hatch tessellation → clip → colour sample → KMeans → extrude glass + lead → GLB
```

Image **decoding stays in JS** (browser-native canvas), so the WASM takes a raw RGBA pixel
buffer in and returns GLB bytes out — no image codec bloat in the module.

## Design notes
- Only deps: `wasm-bindgen` + `serde_json`. No geometry kernel.
- All five hatch patterns produce **convex** cells → triangulation is a trivial fan; clipping is
  Sutherland–Hodgman; the lead came is built as a per-cell **annulus** (tile − inset) so no
  boolean union is needed and the glass holes stay open for backlight.
- Output parity with Python: tile counts match exactly (diamond 266, hexagonal 111, triangle 572,
  brick 133) and the GLB has the same 9 groups (8 translucent glass colours + 1 lead).
- `.pat` import isn't ported yet (needs non-convex `polygonize`); procedural patterns only.

## Build
```powershell
# one-time: rustup target add wasm32-unknown-unknown ; cargo install wasm-bindgen-cli
./build.ps1            # -> pkg/ (ES module, for the web app) and pkg-node/ (for tests)
```

## API
```ts
generate_glb(
  pixels: Uint8Array, img_w, img_h, channels,   // RGBA from canvas (channels = 4)
  pattern: 'rectangle'|'brick'|'diamond'|'hexagonal'|'triangle',
  panel_w, panel_h, tile_size, n_colors,
  lead_gap, glass_depth, frame_height, glass_alpha, seed
): Uint8Array                                    // GLB bytes
count_tiles(pattern, panel_w, panel_h, tile_size): number
```

## Browser usage (feeds the WebGPU renderer)
```js
import init, { generate_glb } from './pkg/sg_wasm.js';
await init();

const img = await createImageBitmap(await (await fetch('window.jpg')).blob());
const cv = Object.assign(document.createElement('canvas'), { width: img.width, height: img.height });
const ctx = cv.getContext('2d'); ctx.drawImage(img, 0, 0);
const { data, width, height } = ctx.getImageData(0, 0, img.width, img.height); // RGBA

const glb = generate_glb(data, width, height, 4,
    'diamond', 200, 300, 16, 8, 1.2, 4.0, 8.0, 0.55, 42);

const url = URL.createObjectURL(new Blob([glb], { type: 'model/gltf-binary' }));
new GLTFLoader().load(url, (g) => scene.add(g.scene));   // three.js / WebGPU
```

## Performance
~3–4 ms warm for a 266-tile diamond panel (vs ~210 ms for the Python pipeline) — roughly 50–60×,
and fully in-browser with no GC churn, so it's suitable for live configurator interaction.

## Test
```powershell
.venv\Scripts\python.exe test\dump_img.py   # decode the sample image to test/img.rgba
node test\run.js                            # run pipeline, write GLBs + timings
.venv\Scripts\python.exe test\validate.py   # check GLB structure + render test/rust_front.png
```

Software, technologies & libraries

  The tools (software)

  ┌───────────────────────┬─────────────────────────────────────────────────────────────────────────────┐
  │         Tool          │                                  Used for                                   │
  ├───────────────────────┼─────────────────────────────────────────────────────────────────────────────┤
  │ Rhino 8 + Grasshopper │ First geometry engine — image sampling, colour quantization, panelization,  │
  │                       │ BREP frames/extrusion (the gh/ and gh/v8/ definitions)                      │
  ├───────────────────────┼─────────────────────────────────────────────────────────────────────────────┤
  │ Speckle (GH           │ Early model transport from Grasshopper to the web viewer                    │
  │ connector)            │                                                                             │
  ├───────────────────────┼─────────────────────────────────────────────────────────────────────────────┤
  │ Python / Jupyter      │ Prototyping the real algorithm outside CAD                                  │
  │                       │ (pipeline/stained_glass_hatch.ipynb)                                        │
  ├───────────────────────┼─────────────────────────────────────────────────────────────────────────────┤
  │ Rust + wasm-bindgen   │ The production engine, compiled to WebAssembly (sg-wasm/)                   │
  ├───────────────────────┼─────────────────────────────────────────────────────────────────────────────┤
  │ three.js (WebGPU)     │ The 3D "beauty" renderer                                                    │
  ├───────────────────────┼─────────────────────────────────────────────────────────────────────────────┤
  │ Cloudflare + Wrangler │ Hosting (no-build static site)                                              │
  ├───────────────────────┼─────────────────────────────────────────────────────────────────────────────┤
  │ Illustrator/Photoshop │ Logo (star rosette), grasshopper mascot, loading GIF, app mockups           │
  └───────────────────────┴─────────────────────────────────────────────────────────────────────────────┘

  Libraries, by layer

  1. Algorithm prototype — Python (pipeline/requirements.txt)
  numpy, pillow (image I/O), shapely (polygon clipping), scikit-learn (KMeans colour quantization), trimesh
  + mapbox-earcut (triangulation) + manifold3d (robust booleans/extrude), matplotlib (debug viz). This
  proved the pipeline: image → hatch tessellation → clip → colour-sample → KMeans → extrude glass + lead →
  GLB.

  2. Production engine — Rust → WASM (sg-wasm/Cargo.toml)
  Deliberately dependency-light: only wasm-bindgen + serde_json, no geometry kernel. They re-derived the
  geometry by hand so it stays tiny and fast:
  - KMeans palette (kmeans.rs), raster sampling (raster.rs), tessellation/mesh (geom.rs, mesh.rs), GLB
  writer (glb.rs)
  - All 5 patterns produce convex cells → triangulation is a trivial fan; clipping is Sutherland–Hodgman;
  the lead came is a per-cell annulus (tile − inset) so no boolean union is needed.
  - Image decoding stays in JS (canvas) — WASM takes raw RGBA in, returns GLB bytes out (no image-codec
  bloat).
  - Verified parity with the Python version (identical tile counts, e.g. diamond 266; same 9 GLB groups) and
  ~3–4 ms vs ~210 ms → 50–60× faster, fully in-browser.

  3. Web front-end (no framework, no build step)
  - three.js 0.184 — WebGPU build + TSL (Three.js Shading Language), self-hosted & minified
  (three.webgpu.min.js 637 KB vs 2 MB), addons: GLTFLoader, GLTFExporter, OrbitControls.
  - @speckle/objectloader early (streaming from Speckle) — later removed; the renderer now builds geometry
  from WASM render_glb → GLTFLoader.parseAsync.
  - Vanilla HTML/CSS/JS: CSS scroll-snap, a custom PL/EN i18n + scroll-jacking script, Google Fonts (Abril
  Fatface, Playfair Display, Lato).
  - Cloudflare static assets via Wrangler (wrangler.jsonc).

  Final data flow (zero backend, all client-side)

  photo (canvas RGBA) → sg-wasm (KMeans + tessellate + extrude) → GLB
          ├─ Configurator: render_preview → 2D canvas (live)
          └─ Renderer: render_glb → GLTFLoader → three.js WebGPU (3D, + .glb download)

  ---
  How the development process looked

  Parallel tracks from the start. The README split the team into image-processing (Somaya, Mateusz),
  geometry (Martyna, Ewa), and web/render/deploy (Bartłomiej) — and the commit graph shows exactly that: GH
  definitions, design assets, and the website all advancing concurrently, repeatedly merged into main.

  Prototype-then-port. Rather than commit to one runtime, we iterated the algorithm where it was fastest
  to explore:
  1. Grasshopper to get the geometry right visually (colour quantization → panels → frames → extrude).
  2. Python notebook to nail the algorithm in code (KMeans, Shapely clipping, trimesh extrude).
  3. Rust/WASM as the final port — same outputs, 50–60× faster, no dependencies, runs in the browser.

  Two decisive pivots — each removing a dependency:
  - Hour ~6: dropped ShapeDiver (wrong fit for a curated configurator) → Speckle + a custom viewer.
  - Hour ~21: the "complete refactor of the pipeline" — replaced Grasshopper + Speckle at runtime with the
  in-browser Rust/WASM engine. This hit their own README's "Stage Three (optional): if possible remove
  ShapeDiver and GH dependency."

  Verification-driven. The WASM port was checked for output parity against the Python reference (tile
  counts, GLB group structure), and the renderer/configurator were exercised end-to-end (live preview, GLB
  export).

  Build philosophy: mobile-first, no build tooling, self-hosted dependencies → a fast, free, static deploy
  that survives offline and has no server to fail during the demo.

  ---
  The story in one line for the slide: We prototyped in Grasshopper, validated in Python, and shipped in
  Rust/WebAssembly — turning a CAD-plus-cloud parametric pipeline into a single static web app that
  generates printable stained glass live in the browser, 50× faster and with zero backend.

  Want me to fold this into the earlier PRESENTATION.md (process + tech sections together), or keep it as a
  separate "Tech" slide
/* tslint:disable */
/* eslint-disable */

/**
 * Compute an `n_colors` palette from the image (downsampled KMeans). Returns flat RGB bytes.
 */
export function compute_palette(pixels: Uint8Array, iw: number, ih: number, ch: number, n_colors: number, seed: number): Uint8Array;

/**
 * Report tile count for a config (for tuning).
 */
export function count_tiles(pattern: string, panel_w: number, panel_h: number, tile_size: number): number;

/**
 * Build the 3D stained-glass GLB using a supplied palette. Returns GLB bytes.
 */
export function render_glb(pixels: Uint8Array, img_w: number, img_h: number, channels: number, pattern: string, panel_w: number, panel_h: number, tile_size: number, lead_gap: number, glass_depth: number, frame_height: number, glass_alpha: number, palette: Uint8Array, merge_cells: boolean): Uint8Array;

/**
 * Rasterize the flat front view using a supplied palette. Returns RGBA (out_w x out_h).
 */
export function render_preview(pixels: Uint8Array, img_w: number, img_h: number, channels: number, pattern: string, panel_w: number, panel_h: number, tile_size: number, lead_gap: number, glass_alpha: number, palette: Uint8Array, out_w: number, out_h: number, merge_cells: boolean): Uint8Array;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
    readonly memory: WebAssembly.Memory;
    readonly compute_palette: (a: number, b: number, c: number, d: number, e: number, f: number, g: number) => [number, number];
    readonly count_tiles: (a: number, b: number, c: number, d: number, e: number) => number;
    readonly render_glb: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number, j: number, k: number, l: number, m: number, n: number, o: number, p: number, q: number) => [number, number];
    readonly render_preview: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number, j: number, k: number, l: number, m: number, n: number, o: number, p: number, q: number) => [number, number];
    readonly __wbindgen_externrefs: WebAssembly.Table;
    readonly __wbindgen_malloc: (a: number, b: number) => number;
    readonly __wbindgen_free: (a: number, b: number, c: number) => void;
    readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
    readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;

/**
 * Instantiates the given `module`, which can either be bytes or
 * a precompiled `WebAssembly.Module`.
 *
 * @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
 *
 * @returns {InitOutput}
 */
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
 * If `module_or_path` is {RequestInfo} or {URL}, makes a request and
 * for everything else, calls `WebAssembly.instantiate` directly.
 *
 * @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
 *
 * @returns {Promise<InitOutput>}
 */
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;

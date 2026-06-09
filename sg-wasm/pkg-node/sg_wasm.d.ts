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

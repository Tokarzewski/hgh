//! Stained-glass hatch pipeline in Rust/WASM.
//! image pixels -> hatch tessellation -> clip -> colour sample -> palette assign -> extrude / rasterize.
//!
//! Palette is computed from the IMAGE (`compute_palette`) so it is stable across pattern/size
//! changes and can be overridden per-colour by the caller, then passed back into the renderers.

mod geom;
mod glb;
mod kmeans;
mod mesh;
mod raster;

use glb::{write_glb, Group};
use mesh::Mesh;
use std::collections::BTreeMap;
use wasm_bindgen::prelude::*;

/// Sample a pixel colour at panel coords (y-up). Returns RGB as f64.
fn sample(px: &[u8], iw: u32, ih: u32, ch: u32, c: [f64; 2], w: f64, h: f64) -> [f64; 3] {
    let u = (c[0] / w).clamp(0.0, 1.0);
    let v = (c[1] / h).clamp(0.0, 1.0);
    let col = ((u * iw as f64) as u32).min(iw - 1);
    let row = (((1.0 - v) * ih as f64) as u32).min(ih - 1); // image row 0 = top
    let i = ((row * iw + col) * ch) as usize;
    [px[i] as f64, px[i + 1] as f64, px[i + 2] as f64]
}

fn collect_tiles(pattern: &str, w: f64, h: f64, tile_size: f64) -> Vec<geom::Poly> {
    let min_area = tile_size * tile_size * 0.02;
    let mut tiles = Vec::new();
    for raw in geom::hatch(pattern, w, h, tile_size) {
        let c = geom::clip_rect(&raw, w, h);
        if c.len() >= 3 && geom::signed_area(&c).abs() > min_area {
            tiles.push(geom::ensure_ccw(&c));
        }
    }
    tiles
}

fn parse_pal(palette: &[u8]) -> Vec<[u8; 3]> {
    palette.chunks_exact(3).map(|c| [c[0], c[1], c[2]]).collect()
}

fn nearest(c: [f64; 3], pal: &[[u8; 3]]) -> usize {
    let mut best = 0;
    let mut bd = f64::INFINITY;
    for (i, p) in pal.iter().enumerate() {
        let d = (c[0] - p[0] as f64).powi(2) + (c[1] - p[1] as f64).powi(2) + (c[2] - p[2] as f64).powi(2);
        if d < bd {
            bd = d;
            best = i;
        }
    }
    best
}

/// Compute an `n_colors` palette from the image (downsampled KMeans). Returns flat RGB bytes.
#[wasm_bindgen]
pub fn compute_palette(pixels: &[u8], iw: u32, ih: u32, ch: u32, n_colors: u32, seed: u32) -> Vec<u8> {
    let total = (iw as usize) * (ih as usize);
    if total == 0 {
        return vec![];
    }
    let stride = (total / 4000).max(1);
    let mut data = Vec::with_capacity(total / stride + 1);
    let mut i = 0;
    while i < total {
        let idx = i * ch as usize;
        data.push([pixels[idx] as f64, pixels[idx + 1] as f64, pixels[idx + 2] as f64]);
        i += stride;
    }
    let (pal, _) = kmeans::kmeans(&data, n_colors as usize, seed);
    pal.into_iter().flatten().collect()
}

/// Rasterize the flat front view using a supplied palette. Returns RGBA (out_w x out_h).
#[wasm_bindgen]
#[allow(clippy::too_many_arguments)]
pub fn render_preview(
    pixels: &[u8], img_w: u32, img_h: u32, channels: u32, pattern: &str,
    panel_w: f64, panel_h: f64, tile_size: f64, lead_gap: f64, glass_alpha: f64,
    palette: &[u8], out_w: u32, out_h: u32,
) -> Vec<u8> {
    const LEAD: [f64; 3] = [28.0, 28.0, 32.0];
    let mut buf = vec![0u8; (out_w * out_h * 4) as usize];
    for px in buf.chunks_mut(4) {
        px.copy_from_slice(&[28, 28, 32, 255]); // lead background
    }
    let pal = parse_pal(palette);
    if pal.is_empty() {
        return buf;
    }
    let a = glass_alpha.clamp(0.0, 1.0);
    let tiles = collect_tiles(pattern, panel_w, panel_h, tile_size);
    let (sx, sy) = (out_w as f64 / panel_w, out_h as f64 / panel_h);
    for t in &tiles {
        let c = sample(pixels, img_w, img_h, channels, geom::centroid(t), panel_w, panel_h);
        let k = nearest(c, &pal);
        // composite glass colour over the lead backing by alpha (preview of transparency)
        let col = pal[k];
        let shown = [
            (col[0] as f64 * a + LEAD[0] * (1.0 - a)).round() as u8,
            (col[1] as f64 * a + LEAD[1] * (1.0 - a)).round() as u8,
            (col[2] as f64 * a + LEAD[2] * (1.0 - a)).round() as u8,
        ];
        if let Some(inset) = geom::inset_convex(t, lead_gap) {
            let poly: Vec<[f64; 2]> =
                inset.iter().map(|p| [p[0] * sx, (panel_h - p[1]) * sy]).collect(); // flip Y
            raster::fill_poly(&mut buf, out_w, out_h, &poly, shown);
        }
    }
    buf
}

/// Build the 3D stained-glass GLB using a supplied palette. Returns GLB bytes.
#[wasm_bindgen]
#[allow(clippy::too_many_arguments)]
pub fn render_glb(
    pixels: &[u8], img_w: u32, img_h: u32, channels: u32, pattern: &str,
    panel_w: f64, panel_h: f64, tile_size: f64, lead_gap: f64,
    glass_depth: f64, frame_height: f64, glass_alpha: f64, palette: &[u8],
) -> Vec<u8> {
    let pal = parse_pal(palette);
    if pal.is_empty() {
        return write_glb(&[]);
    }
    let tiles = collect_tiles(pattern, panel_w, panel_h, tile_size);
    let gz0 = (frame_height - glass_depth) * 0.5;
    let gz1 = gz0 + glass_depth;
    let alpha = (glass_alpha * 255.0).round().clamp(0.0, 255.0) as u8;

    let mut glass: BTreeMap<usize, Mesh> = BTreeMap::new();
    let mut lead = Mesh::default();
    for t in &tiles {
        let c = sample(pixels, img_w, img_h, channels, geom::centroid(t), panel_w, panel_h);
        let k = nearest(c, &pal);
        if let Some(inset) = geom::inset_convex(t, lead_gap) {
            glass.entry(k).or_default().extrude(&inset, gz0, gz1);
            lead.annulus(t, &inset, 0.0, frame_height);
        }
    }
    let mut groups: Vec<Group> = Vec::new();
    for (k, m) in glass {
        let c = pal[k];
        groups.push(Group { mesh: m, color: [c[0], c[1], c[2], alpha], metallic: 0.0, roughness: 0.12 });
    }
    if !lead.is_empty() {
        groups.push(Group { mesh: lead, color: [28, 28, 32, 255], metallic: 0.55, roughness: 0.45 });
    }
    write_glb(&groups)
}

/// Report tile count for a config (for tuning).
#[wasm_bindgen]
pub fn count_tiles(pattern: &str, panel_w: f64, panel_h: f64, tile_size: f64) -> u32 {
    collect_tiles(pattern, panel_w, panel_h, tile_size).len() as u32
}

//! Scanline polygon fill for the 2D front-view preview (no GPU needed).
use crate::geom::Pt;

/// Fill a polygon (pixel coords) into an RGBA buffer with a flat colour.
pub fn fill_poly(buf: &mut [u8], w: u32, h: u32, poly: &[Pt], color: [u8; 3]) {
    let n = poly.len();
    if n < 3 {
        return;
    }
    let (mut ymin, mut ymax) = (f64::INFINITY, f64::NEG_INFINITY);
    for p in poly {
        ymin = ymin.min(p[1]);
        ymax = ymax.max(p[1]);
    }
    let y0 = (ymin.floor().max(0.0)) as i32;
    let y1 = (ymax.ceil().min(h as f64 - 1.0)) as i32;
    let mut xs: Vec<f64> = Vec::with_capacity(n);
    for y in y0..=y1 {
        let yc = y as f64 + 0.5;
        xs.clear();
        for i in 0..n {
            let a = poly[i];
            let b = poly[(i + 1) % n];
            let (ay, by) = (a[1], b[1]);
            if (ay <= yc && by > yc) || (by <= yc && ay > yc) {
                let t = (yc - ay) / (by - ay);
                xs.push(a[0] + t * (b[0] - a[0]));
            }
        }
        if xs.len() < 2 {
            continue;
        }
        xs.sort_by(|p, q| p.partial_cmp(q).unwrap());
        let mut k = 0;
        while k + 1 < xs.len() {
            let xa = (xs[k].ceil().max(0.0)) as i32;
            let xb = (xs[k + 1].floor().min(w as f64 - 1.0)) as i32;
            for x in xa..=xb {
                let idx = ((y as u32 * w + x as u32) * 4) as usize;
                buf[idx] = color[0];
                buf[idx + 1] = color[1];
                buf[idx + 2] = color[2];
                buf[idx + 3] = 255;
            }
            k += 2;
        }
    }
}

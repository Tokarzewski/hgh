//! Dependency-free 2D geometry: hatch generators, rectangle clip, convex inset.

pub type Pt = [f64; 2];
pub type Poly = Vec<Pt>;

pub fn signed_area(p: &[Pt]) -> f64 {
    let n = p.len();
    let mut a = 0.0;
    for i in 0..n {
        let j = (i + 1) % n;
        a += p[i][0] * p[j][1] - p[j][0] * p[i][1];
    }
    a * 0.5
}

pub fn ensure_ccw(p: &[Pt]) -> Poly {
    if signed_area(p) < 0.0 {
        p.iter().rev().cloned().collect()
    } else {
        p.to_vec()
    }
}

pub fn centroid(p: &[Pt]) -> Pt {
    let (mut x, mut y) = (0.0, 0.0);
    for v in p {
        x += v[0];
        y += v[1];
    }
    let n = p.len() as f64;
    [x / n, y / n]
}

// ---- Sutherland-Hodgman clip of a convex/any polygon against axis rect [0,W]x[0,H] ----
fn clip_edge(poly: &[Pt], keep: impl Fn(Pt) -> bool, isect: impl Fn(Pt, Pt) -> Pt) -> Poly {
    let mut out = Vec::with_capacity(poly.len() + 4);
    let n = poly.len();
    if n == 0 {
        return out;
    }
    for i in 0..n {
        let cur = poly[i];
        let prev = poly[(i + n - 1) % n];
        let (ci, pi) = (keep(cur), keep(prev));
        if ci {
            if !pi {
                out.push(isect(prev, cur));
            }
            out.push(cur);
        } else if pi {
            out.push(isect(prev, cur));
        }
    }
    out
}

pub fn clip_rect(poly: &[Pt], w: f64, h: f64) -> Poly {
    let lerp = |a: Pt, b: Pt, t: f64| [a[0] + (b[0] - a[0]) * t, a[1] + (b[1] - a[1]) * t];
    let mut p = poly.to_vec();
    // x >= 0
    p = clip_edge(&p, |q| q[0] >= 0.0, |a, b| lerp(a, b, (0.0 - a[0]) / (b[0] - a[0])));
    if p.is_empty() { return p; }
    // x <= w
    p = clip_edge(&p, |q| q[0] <= w, |a, b| lerp(a, b, (w - a[0]) / (b[0] - a[0])));
    if p.is_empty() { return p; }
    // y >= 0
    p = clip_edge(&p, |q| q[1] >= 0.0, |a, b| lerp(a, b, (0.0 - a[1]) / (b[1] - a[1])));
    if p.is_empty() { return p; }
    // y <= h
    clip_edge(&p, |q| q[1] <= h, |a, b| lerp(a, b, (h - a[1]) / (b[1] - a[1])))
}

fn line_isect(p1: Pt, d1: Pt, p2: Pt, d2: Pt) -> Option<Pt> {
    let denom = d1[0] * d2[1] - d1[1] * d2[0];
    if denom.abs() < 1e-12 {
        return None;
    }
    let t = ((p2[0] - p1[0]) * d2[1] - (p2[1] - p1[1]) * d2[0]) / denom;
    Some([p1[0] + t * d1[0], p1[1] + t * d1[1]])
}

/// Inward edge-offset inset of a convex polygon by `g`. Returns None if it collapses.
pub fn inset_convex(poly: &[Pt], g: f64) -> Option<Poly> {
    let p = ensure_ccw(poly);
    let gaps = vec![g; p.len()];
    inset_convex_var(&p, &gaps)
}

/// Per-edge inward inset of a convex CCW polygon: `gaps[i]` offsets edge
/// `p[i] -> p[i+1]`. A gap of 0 leaves that edge in place (used to "merge"
/// neighbouring cells by dropping the lead between them).
pub fn inset_convex_var(p: &[Pt], gaps: &[f64]) -> Option<Poly> {
    let n = p.len();
    if n < 3 || gaps.len() != n {
        return None;
    }
    let mut lines: Vec<(Pt, Pt)> = Vec::with_capacity(n);
    for i in 0..n {
        let a = p[i];
        let b = p[(i + 1) % n];
        let (dx, dy) = (b[0] - a[0], b[1] - a[1]);
        let len = (dx * dx + dy * dy).sqrt();
        if len < 1e-9 {
            return None;
        }
        let d = [dx / len, dy / len];
        let nrm = [-d[1], d[0]]; // inward for CCW
        let pt = [a[0] + gaps[i] * nrm[0], a[1] + gaps[i] * nrm[1]];
        lines.push((pt, d));
    }
    let mut out = Vec::with_capacity(n);
    for i in 0..n {
        let l0 = lines[(i + n - 1) % n];
        let l1 = lines[i];
        out.push(line_isect(l0.0, l0.1, l1.0, l1.1)?);
    }
    if signed_area(&out) <= 1e-6 {
        return None;
    }
    Some(out)
}

/// Point-in-convex-CCW-polygon test (boundary counts as inside).
pub fn contains_convex(p: &[Pt], q: Pt) -> bool {
    let n = p.len();
    for i in 0..n {
        let a = p[i];
        let b = p[(i + 1) % n];
        if (b[0] - a[0]) * (q[1] - a[1]) - (b[1] - a[1]) * (q[0] - a[0]) < -1e-9 {
            return false;
        }
    }
    n >= 3
}

// ---------------- hatch generators (all return convex cells) ----------------
fn rotate45(p: Pt, c: Pt) -> Pt {
    let a = std::f64::consts::FRAC_PI_4;
    let (ca, sa) = (a.cos(), a.sin());
    let (x, y) = (p[0] - c[0], p[1] - c[1]);
    [c[0] + x * ca - y * sa, c[1] + x * sa + y * ca]
}

pub fn hatch(pattern: &str, w: f64, h: f64, s: f64) -> Vec<Poly> {
    match pattern {
        "rectangle" => grid(w, h, s, s, false),
        "brick" => brick(w, h, s),
        "diamond" => diamond(w, h, s),
        "hexagonal" => hexagon(w, h, s),
        "triangle" => triangle(w, h, s),
        _ => diamond(w, h, s),
    }
}

fn grid(w: f64, h: f64, sx: f64, sy: f64, _b: bool) -> Vec<Poly> {
    let mut out = Vec::new();
    let nx = (w / sx) as i32 + 2;
    let ny = (h / sy) as i32 + 2;
    for i in -1..nx {
        for j in -1..ny {
            let (x, y) = (i as f64 * sx, j as f64 * sy);
            out.push(vec![[x, y], [x + sx, y], [x + sx, y + sy], [x, y + sy]]);
        }
    }
    out
}

fn brick(w: f64, h: f64, s: f64) -> Vec<Poly> {
    let (bw, bh) = (2.0 * s, s);
    let mut out = Vec::new();
    let ny = (h / bh) as i32 + 2;
    for j in -1..ny {
        let off = if j.rem_euclid(2) == 1 { bw / 2.0 } else { 0.0 };
        let nx = (w / bw) as i32 + 3;
        for i in -1..nx {
            let x = i as f64 * bw - off;
            let y = j as f64 * bh;
            out.push(vec![[x, y], [x + bw, y], [x + bw, y + bh], [x, y + bh]]);
        }
    }
    out
}

fn diamond(w: f64, h: f64, s: f64) -> Vec<Poly> {
    let pad = 0.75 * w.max(h);
    let c = [w / 2.0, h / 2.0];
    let i0 = (-pad / s) as i32 - 1;
    let i1 = ((w + pad) / s) as i32 + 1;
    let j0 = (-pad / s) as i32 - 1;
    let j1 = ((h + pad) / s) as i32 + 1;
    let mut out = Vec::new();
    for i in i0..i1 {
        for j in j0..j1 {
            let (x, y) = (i as f64 * s, j as f64 * s);
            let sq = [[x, y], [x + s, y], [x + s, y + s], [x, y + s]];
            out.push(sq.iter().map(|&p| rotate45(p, c)).collect());
        }
    }
    out
}

fn hexagon(w: f64, h: f64, s: f64) -> Vec<Poly> {
    let ww = (3.0_f64).sqrt() * s;
    let vstep = 1.5 * s;
    let mut out = Vec::new();
    let nr = (h / vstep) as i32 + 3;
    let nc = (w / ww) as i32 + 3;
    for j in -1..nr {
        for i in -1..nc {
            let cx = i as f64 * ww + if j.rem_euclid(2) == 1 { ww / 2.0 } else { 0.0 };
            let cy = j as f64 * vstep;
            let mut hexp = Vec::with_capacity(6);
            for k in 0..6 {
                let a = (60.0 * k as f64 + 90.0).to_radians();
                hexp.push([cx + s * a.cos(), cy + s * a.sin()]);
            }
            out.push(hexp);
        }
    }
    out
}

fn triangle(w: f64, h: f64, s: f64) -> Vec<Poly> {
    let th = s * (3.0_f64).sqrt() / 2.0;
    let mut out = Vec::new();
    let nj = (h / th) as i32 + 2;
    let ni = (w / s) as i32 + 3;
    for j in -1..nj {
        let (y0, y1) = (j as f64 * th, (j + 1) as f64 * th);
        for i in -1..ni {
            let x = i as f64 * s;
            out.push(vec![[x, y0], [x + s, y0], [x + s / 2.0, y1]]);
            out.push(vec![[x + s / 2.0, y1], [x + 1.5 * s, y1], [x + s, y0]]);
        }
    }
    out
}

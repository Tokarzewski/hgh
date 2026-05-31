//! Triangle-soup mesh accumulator + convex prism / lead-annulus extrusion.
use crate::geom::Pt;

#[derive(Default)]
pub struct Mesh {
    pub pos: Vec<f32>, // xyz
    pub nrm: Vec<f32>, // xyz
    pub idx: Vec<u32>,
}

impl Mesh {
    fn tri(&mut self, a: [f64; 3], b: [f64; 3], c: [f64; 3]) {
        let u = [b[0] - a[0], b[1] - a[1], b[2] - a[2]];
        let v = [c[0] - a[0], c[1] - a[1], c[2] - a[2]];
        let mut n = [
            u[1] * v[2] - u[2] * v[1],
            u[2] * v[0] - u[0] * v[2],
            u[0] * v[1] - u[1] * v[0],
        ];
        let len = (n[0] * n[0] + n[1] * n[1] + n[2] * n[2]).sqrt();
        if len > 1e-12 {
            n = [n[0] / len, n[1] / len, n[2] / len];
        }
        for p in [a, b, c] {
            let base = (self.pos.len() / 3) as u32;
            self.pos.extend_from_slice(&[p[0] as f32, p[1] as f32, p[2] as f32]);
            self.nrm.extend_from_slice(&[n[0] as f32, n[1] as f32, n[2] as f32]);
            self.idx.push(base);
        }
    }

    fn quad(&mut self, a: [f64; 3], b: [f64; 3], c: [f64; 3], d: [f64; 3]) {
        self.tri(a, b, c);
        self.tri(a, c, d);
    }

    /// Extrude a convex CCW polygon (XY) into a prism spanning z in [z0, z1].
    pub fn extrude(&mut self, poly: &[Pt], z0: f64, z1: f64) {
        let n = poly.len();
        if n < 3 {
            return;
        }
        let p3 = |i: usize, z: f64| [poly[i][0], poly[i][1], z];
        // top (z1) fan, faces +Z
        for i in 1..n - 1 {
            self.tri(p3(0, z1), p3(i, z1), p3(i + 1, z1));
        }
        // bottom (z0) fan, reversed -> faces -Z
        for i in 1..n - 1 {
            self.tri(p3(0, z0), p3(i + 1, z0), p3(i, z0));
        }
        // sides
        for i in 0..n {
            let j = (i + 1) % n;
            self.quad(p3(i, z0), p3(j, z0), p3(j, z1), p3(i, z1));
        }
    }

    /// Extrude a frame (annulus) between outer and inner rings (same vertex count, both CCW).
    /// Leaves the inner hole open so backlight passes through the glass that fills it.
    pub fn annulus(&mut self, outer: &[Pt], inner: &[Pt], z0: f64, z1: f64) {
        let n = outer.len();
        if n < 3 || inner.len() != n {
            return;
        }
        let o = |i: usize, z: f64| [outer[i][0], outer[i][1], z];
        let ii = |i: usize, z: f64| [inner[i][0], inner[i][1], z];
        for i in 0..n {
            let j = (i + 1) % n;
            // top strip (faces +Z)
            self.quad(o(i, z1), o(j, z1), ii(j, z1), ii(i, z1));
            // bottom strip (faces -Z)
            self.quad(o(i, z0), ii(i, z0), ii(j, z0), o(j, z0));
            // outer wall (faces out)
            self.quad(o(i, z0), o(j, z0), o(j, z1), o(i, z1));
            // inner wall (faces into the hole)
            self.quad(ii(j, z0), ii(i, z0), ii(i, z1), ii(j, z1));
        }
    }

    pub fn is_empty(&self) -> bool {
        self.idx.is_empty()
    }
}

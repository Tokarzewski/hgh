//! Tiny deterministic KMeans (Lloyd + k-means++ seeding) for RGB palette quantization.

struct Lcg(u64);
impl Lcg {
    fn next_f(&mut self) -> f64 {
        self.0 = self.0.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        ((self.0 >> 11) as f64) / ((1u64 << 53) as f64)
    }
}

fn dist2(a: [f64; 3], b: [f64; 3]) -> f64 {
    let (dx, dy, dz) = (a[0] - b[0], a[1] - b[1], a[2] - b[2]);
    dx * dx + dy * dy + dz * dz
}

/// Returns (palette, labels). Palette has at most `k` colors.
pub fn kmeans(data: &[[f64; 3]], k: usize, seed: u32) -> (Vec<[u8; 3]>, Vec<usize>) {
    let n = data.len();
    if n == 0 {
        return (vec![], vec![]);
    }
    let k = k.max(1).min(n);
    let mut rng = Lcg(seed as u64 ^ 0x9E3779B97F4A7C15);

    // k-means++ seeding
    let mut cent: Vec<[f64; 3]> = Vec::with_capacity(k);
    cent.push(data[(rng.next_f() * n as f64) as usize % n]);
    while cent.len() < k {
        let mut d2: Vec<f64> = data
            .iter()
            .map(|&p| cent.iter().map(|&c| dist2(p, c)).fold(f64::INFINITY, f64::min))
            .collect();
        let sum: f64 = d2.iter().sum();
        if sum <= 0.0 {
            break;
        }
        let mut t = rng.next_f() * sum;
        let mut chosen = n - 1;
        for (i, w) in d2.iter_mut().enumerate() {
            t -= *w;
            if t <= 0.0 {
                chosen = i;
                break;
            }
        }
        cent.push(data[chosen]);
    }
    let k = cent.len();

    let mut labels = vec![0usize; n];
    for _ in 0..16 {
        // assign
        let mut changed = false;
        for (i, &p) in data.iter().enumerate() {
            let mut best = 0;
            let mut bd = f64::INFINITY;
            for (c, &cc) in cent.iter().enumerate() {
                let d = dist2(p, cc);
                if d < bd {
                    bd = d;
                    best = c;
                }
            }
            if labels[i] != best {
                labels[i] = best;
                changed = true;
            }
        }
        // update
        let mut acc = vec![[0.0; 3]; k];
        let mut cnt = vec![0.0; k];
        for (i, &p) in data.iter().enumerate() {
            let l = labels[i];
            acc[l][0] += p[0];
            acc[l][1] += p[1];
            acc[l][2] += p[2];
            cnt[l] += 1.0;
        }
        for c in 0..k {
            if cnt[c] > 0.0 {
                cent[c] = [acc[c][0] / cnt[c], acc[c][1] / cnt[c], acc[c][2] / cnt[c]];
            }
        }
        if !changed {
            break;
        }
    }

    let palette = cent
        .iter()
        .map(|c| {
            [
                c[0].round().clamp(0.0, 255.0) as u8,
                c[1].round().clamp(0.0, 255.0) as u8,
                c[2].round().clamp(0.0, 255.0) as u8,
            ]
        })
        .collect();
    (palette, labels)
}

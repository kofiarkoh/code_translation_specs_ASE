use std::io::{self, Read};

const INF: i32 = 0x7fffffff;

struct Seg2D {
    // Power-of-two sizes (leaf grid is h x w)
    h: usize,
    w: usize,
    // Tree dimensions: (2h-1) x (2w-1)
    h2: usize,
    w2: usize,
    // Flattened storage for seg[r][c]
    seg: Vec<i32>,

    // Query state (mirrors the C globals used by query/query2)
    r1: usize,
    c1: usize,
    r2: usize, // exclusive
    c2: usize, // exclusive
    g: usize,  // row-tree node index selected by query()
}

impl Seg2D {
    fn new(r: usize, c: usize) -> Self {
        let mut h = 1usize;
        let mut w = 1usize;
        while h < r {
            h <<= 1;
        }
        while w < c {
            w <<= 1;
        }
        let h2 = (h << 1) - 1;
        let w2 = (w << 1) - 1;

        // C code memset(pool,0) => all zeros initially.
        // Note: This differs from a typical min-segtree initialization (INF),
        // but we preserve C semantics exactly.
        let seg = vec![0i32; h2 * w2];

        Self {
            h,
            w,
            h2,
            w2,
            seg,
            r1: 0,
            c1: 0,
            r2: 0,
            c2: 0,
            g: 0,
        }
    }

    #[inline]
    fn idx(&self, r: usize, c: usize) -> usize {
        r * self.w2 + c
    }

    fn update(&mut self, r: usize, c: usize, x: i32) {
        // Translate C's 0-based leaf mapping: r += h-1, c += w-1
        let mut rr = r + (self.h - 1);
        let mut cc = c + (self.w - 1);
        let w2 = self.w2;

        self.seg[self.idx(rr, cc)] = x;

        loop {
            // Update along the column-tree within this row-tree node.
            let mut c2 = cc;
            while c2 != 0 {
                if (c2 & 1) == 0 {
                    c2 -= 1;
                }
                let t = c2;
                c2 >>= 1;
                let a = self.seg[self.idx(rr, t)];
                let b = self.seg[self.idx(rr, t + 1)];
                self.seg[self.idx(rr, c2)] = if a > b { b } else { a };
            }

            if rr == 0 {
                break;
            }

            // Move up one level in the row-tree, updating the value at (parent, cc).
            if (rr & 1) == 0 {
                rr -= 1;
            }
            let t = rr;
            rr >>= 1;
            let a = self.seg[t * w2 + cc];
            let b = self.seg[(t + 1) * w2 + cc];
            self.seg[rr * w2 + cc] = if a > b { b } else { a };
        }
    }

    fn query2(&self, l: usize, r: usize, k: usize) -> i32 {
        // Column segment tree query on row-node self.g, using [c1,c2) bounds.
        if self.c2 <= l || r <= self.c1 {
            return INF;
        }
        if self.c1 <= l && r <= self.c2 {
            return self.seg[self.idx(self.g, k)];
        }

        let m = (l + r) >> 1;
        let mut kk = k << 1;
        kk += 1;
        let lmin = self.query2(l, m, kk);
        kk += 1;
        let rmin = self.query2(m, r, kk);
        if lmin > rmin { rmin } else { lmin }
    }

    fn query(&mut self, l: usize, r: usize, k: usize) -> i32 {
        // Row segment tree query using [r1,r2) bounds; for covered nodes, query2 is used.
        if self.r2 <= l || r <= self.r1 {
            return INF;
        }
        if self.r1 <= l && r <= self.r2 {
            self.g = k;
            return self.query2(0, self.w, 0);
        }

        let m = (l + r) >> 1;
        let mut kk = k << 1;
        kk += 1;
        let lmin = self.query(l, m, kk);
        kk += 1;
        let rmin = self.query(m, r, kk);
        if lmin > rmin { rmin } else { lmin }
    }

    fn set_query_rect(&mut self, r1: i32, c1: i32, r2: i32, c2: i32) {
        // C reads r1,c1,r2,c2 then does r2++, c2++ (making them exclusive).
        // We preserve that behavior. Negative indices are treated as out-of-bounds;
        // the spec says out-of-bounds queries return a predefined constant (INF).
        // The original C code does not guard against negatives; in Rust we clamp
        // to avoid panics while preserving "out of bounds => INF" behavior.
        let r1e = r1;
        let c1e = c1;
        let r2e = r2.saturating_add(1);
        let c2e = c2.saturating_add(1);

        // Store as usize with clamping to [0, ...] to keep recursion safe.
        self.r1 = if r1e < 0 { 0 } else { r1e as usize };
        self.c1 = if c1e < 0 { 0 } else { c1e as usize };
        self.r2 = if r2e < 0 { 0 } else { r2e as usize };
        self.c2 = if c2e < 0 { 0 } else { c2e as usize };
    }
}

fn main() {
    // Read all input as whitespace-separated integers.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let mut out = String::new();

    loop {
        let r_opt = it.next();
        if r_opt.is_none() {
            break;
        }
        let r: i32 = r_opt.unwrap().parse().unwrap();
        let c: i32 = match it.next() {
            Some(v) => v.parse().unwrap(),
            None => break,
        };
        let q: i32 = match it.next() {
            Some(v) => v.parse().unwrap(),
            None => break,
        };

        // Spec: if R or C are non-positive, terminate without further processing.
        if r <= 0 || c <= 0 {
            break;
        }

        let r_us = r as usize;
        let c_us = c as usize;
        let q_us = if q < 0 { 0 } else { q as usize };

        let mut seg = Seg2D::new(r_us, c_us);

        for rr in 0..r_us {
            for cc in 0..c_us {
                let v: i32 = it.next().unwrap().parse().unwrap();
                seg.update(rr, cc, v);
            }
        }

        for _ in 0..q_us {
            let r1: i32 = it.next().unwrap().parse().unwrap();
            let c1: i32 = it.next().unwrap().parse().unwrap();
            let r2: i32 = it.next().unwrap().parse().unwrap();
            let c2: i32 = it.next().unwrap().parse().unwrap();

            seg.set_query_rect(r1, c1, r2, c2);

            // If the query rectangle is outside the grid, return INF (spec).
            // We also handle cases where the exclusive bounds are inverted.
            let ans = if seg.r1 >= seg.r2 || seg.c1 >= seg.c2 {
                INF
            } else if seg.r1 >= seg.h || seg.c1 >= seg.w {
                INF
            } else {
                // Clamp upper bounds to tree range to avoid unnecessary recursion;
                // out-of-bounds parts are naturally ignored by the segment query.
                if seg.r2 > seg.h {
                    seg.r2 = seg.h;
                }
                if seg.c2 > seg.w {
                    seg.c2 = seg.w;
                }
                seg.query(0, seg.h, 0)
            };

            out.push_str(&format!("{ans}\n"));
        }
    }

    print!("{out}");
}

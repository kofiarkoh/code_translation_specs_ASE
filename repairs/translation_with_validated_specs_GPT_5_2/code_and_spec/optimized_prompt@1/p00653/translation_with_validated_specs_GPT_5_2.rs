use std::io::{self, Read};

const INF: i32 = 0x7fffffff;

struct SegTree2D {
    // Power-of-two sizes (leaf grid is h x w)
    h: usize,
    w: usize,
    // Backing store dimensions for the implicit full binary trees:
    // rows: (2*h - 1), cols: (2*w - 1)
    h2: usize,
    w2: usize,
    // Flattened 2D array: seg[r * w2 + c]
    seg: Vec<i32>,
}

impl SegTree2D {
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

        // Spec says no explicit error handling; we just allocate.
        // Initialize with 0 like the C code's memset(pool, 0, ...).
        let seg = vec![0i32; h2 * w2];

        Self { h, w, h2, w2, seg }
    }

    #[inline]
    fn idx(&self, r: usize, c: usize) -> usize {
        r * self.w2 + c
    }

    fn get(&self, r: usize, c: usize) -> i32 {
        self.seg[self.idx(r, c)]
    }

    fn set(&mut self, r: usize, c: usize, v: i32) {
        let idx = self.idx(r, c);
        self.seg[idx] = v;
    }

    // Mirrors the C update() exactly in structure and indexing.
    fn update(&mut self, r0: usize, c0: usize, x: i32) {
        let mut r = r0 + self.h - 1;
        let mut c = c0 + self.w - 1;
        self.set(r, c, x);

        loop {
            // Build along the column segment tree at fixed row r.
            let mut c2 = c;
            while c2 != 0 {
                if (c2 & 1) == 0 {
                    c2 -= 1;
                }
                let t = c2;
                c2 >>= 1;
                let a = self.get(r, t);
                let b = self.get(r, t + 1);
                self.set(r, c2, if a > b { b } else { a });
            }

            if r == 0 {
                break;
            }

            // Move up one level in the row segment tree, updating the value at column c.
            if (r & 1) == 0 {
                r -= 1;
            }
            let t = r;
            r >>= 1;
            let a = self.get(t, c);
            let b = self.get(t + 1, c);
            self.set(r, c, if a > b { b } else { a });
        }
    }

    // query2 in C: query over columns for a fixed row-node g.
    fn query2(&self, g: usize, c1: usize, c2: usize, l: usize, r: usize, k: usize) -> i32 {
        if c2 <= l || r <= c1 {
            return INF;
        }
        if c1 <= l && r <= c2 {
            return self.get(g, k);
        }
        let m = (l + r) >> 1;
        let mut kk = k << 1;
        kk += 1;
        let lmin = self.query2(g, c1, c2, l, m, kk);
        kk += 1;
        let rmin = self.query2(g, c1, c2, m, r, kk);
        if lmin > rmin { rmin } else { lmin }
    }

    // query in C: query over rows; when a row-node is fully covered, delegate to query2.
    fn query(&self, r1: usize, r2: usize, c1: usize, c2: usize, l: usize, r: usize, k: usize) -> i32 {
        if r2 <= l || r <= r1 {
            return INF;
        }
        if r1 <= l && r <= r2 {
            // g = k in C
            return self.query2(k, c1, c2, 0, self.w, 0);
        }
        let m = (l + r) >> 1;
        let mut kk = k << 1;
        kk += 1;
        let lmin = self.query(r1, r2, c1, c2, l, m, kk);
        kk += 1;
        let rmin = self.query(r1, r2, c1, c2, m, r, kk);
        if lmin > rmin { rmin } else { lmin }
    }
}

fn main() {
    // Read all input; behavior on malformed input is unspecified per spec.
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();
    let mut it = s.split_whitespace();

    let mut out = String::new();

    loop {
        let r_opt = it.next();
        if r_opt.is_none() {
            break;
        }
        let r: i32 = r_opt.unwrap().parse().unwrap();
        let c: i32 = it.next().unwrap().parse().unwrap();
        let q: i32 = it.next().unwrap().parse().unwrap();

        if r <= 0 {
            break;
        }

        let r_us = r as usize;
        let c_us = c as usize;
        let q_us = q as usize;

        let mut st = SegTree2D::new(r_us, c_us);

        for rr in 0..r_us {
            for cc in 0..c_us {
                let v: i32 = it.next().unwrap().parse().unwrap();
                st.update(rr, cc, v);
            }
        }

        for _ in 0..q_us {
            let r1: usize = it.next().unwrap().parse().unwrap();
            let c1: usize = it.next().unwrap().parse().unwrap();
            let r2_in: usize = it.next().unwrap().parse().unwrap();
            let c2_in: usize = it.next().unwrap().parse().unwrap();

            // C code treats input as inclusive corners and then does r2++, c2++
            // to convert to half-open [r1, r2) and [c1, c2).
            let r2 = r2_in + 1;
            let c2 = c2_in + 1;

            let ans = st.query(r1, r2, c1, c2, 0, st.h, 0);
            out.push_str(&format!("{ans}\n"));
        }
    }

    print!("{out}");
}

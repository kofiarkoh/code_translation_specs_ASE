use std::io::{self, Read};

const INF: i32 = 0x7fffffff;

struct SegTree2D {
    // Stored as a flat array representing seg[r][c] with row-major layout.
    // Dimensions are h2 x w2 where:
    //   h2 = 2*h - 1, w2 = 2*w - 1
    seg: Vec<i32>,
    h: usize,
    w: usize,
    h2: usize,
    w2: usize,

    // Query parameters (match C globals)
    r1: i32,
    c1: i32,
    r2: i32,
    c2: i32,
    g: usize,
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

        // C code zero-initializes pool; we do the same.
        let seg = vec![0i32; h2 * w2];

        Self {
            seg,
            h,
            w,
            h2,
            w2,
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

    fn update(&mut self, r0: usize, c0: usize, x: i32) {
        // Direct translation of the C update() logic.
        let mut r = (r0 + self.h - 1) as i32;
        let mut c = (c0 + self.w - 1) as i32;

        self.seg[self.idx(r as usize, c as usize)] = x;

        while r >= 0 {
            let mut c2 = c;
            while c2 != 0 {
                if (c2 & 1) == 0 {
                    c2 -= 1;
                }
                let mut t = c2;
                c2 >>= 1;

                let a1 = self.seg[self.idx(r as usize, t as usize)];
                t += 1;
                let a2 = self.seg[self.idx(r as usize, t as usize)];
                let a = if a1 > a2 { a2 } else { a1 };
                self.seg[self.idx(r as usize, c2 as usize)] = a;
            }

            if r == 0 {
                break;
            }

            if (r & 1) == 0 {
                r -= 1;
            }
            let mut t = r;
            r >>= 1;

            let a1 = self.seg[self.idx(t as usize, c as usize)];
            t += 1;
            let a2 = self.seg[self.idx(t as usize, c as usize)];
            let a = if a1 > a2 { a2 } else { a1 };
            self.seg[self.idx(r as usize, c as usize)] = a;
        }
    }

    fn query2(&mut self, l: i32, r: i32, k: usize) -> i32 {
        // Direct translation of C query2(), using self.c1/self.c2 and self.g.
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

    fn query(&mut self, l: i32, r: i32, k: usize) -> i32 {
        // Direct translation of C query(), using self.r1/self.r2 and setting self.g.
        if self.r2 <= l || r <= self.r1 {
            return INF;
        }
        if self.r1 <= l && r <= self.r2 {
            self.g = k;
            return self.query2(0, self.w as i32, 0);
        }

        let m = (l + r) >> 1;
        let mut kk = k << 1;
        kk += 1;
        let lmin = self.query(l, m, kk);
        kk += 1;
        let rmin = self.query(m, r, kk);
        if lmin > rmin { rmin } else { lmin }
    }
}

fn main() {
    // Read all input and parse as integers (scanf-like whitespace handling).
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
        let c: i32 = it.next().unwrap().parse().unwrap();
        let q: i32 = it.next().unwrap().parse().unwrap();

        if r <= 0 {
            break;
        }

        let r_us = r as usize;
        let c_us = c as usize;
        let mut st = SegTree2D::new(r_us, c_us);

        for rr in 0..r_us {
            for cc in 0..c_us {
                let v: i32 = it.next().unwrap().parse().unwrap();
                st.update(rr, cc, v);
            }
        }

        for _ in 0..(q as usize) {
            st.r1 = it.next().unwrap().parse().unwrap();
            st.c1 = it.next().unwrap().parse().unwrap();
            st.r2 = it.next().unwrap().parse().unwrap();
            st.c2 = it.next().unwrap().parse().unwrap();
            // C code does r2++, c2++ (inclusive -> half-open)
            st.r2 += 1;
            st.c2 += 1;

            let ans = st.query(0, st.h as i32, 0);
            out.push_str(&format!("{ans}\n"));
        }
    }

    print!("{out}");
}

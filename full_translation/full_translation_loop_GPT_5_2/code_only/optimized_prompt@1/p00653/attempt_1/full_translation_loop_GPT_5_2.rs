use std::io::{self, Read};

const INF: i32 = 0x7fffffff;

struct SegTree2D {
    // Dimensions are powers of two (h for rows, w for cols)
    h: usize,
    w: usize,
    // Flattened storage for (2h-1) x (2w-1) segment tree nodes.
    // Indexing: seg[r * w2 + c]
    w2: usize,
    seg: Vec<i32>,

    // Query parameters (mirrors C globals)
    r1: usize,
    c1: usize,
    r2: usize,
    c2: usize,
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
        // C code memset(pool,0) then builds seg pointers into it.
        // We emulate with a zero-initialized Vec.
        let seg = vec![0i32; h2 * w2];

        Self {
            h,
            w,
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

    fn update(&mut self, r0: usize, c0: usize, x: i32) {
        // Direct translation of the C logic, including the somewhat unusual
        // parent recomputation order and index arithmetic.
        let mut r = r0 + self.h - 1;
        let mut c = c0 + self.w - 1;
        let w2 = self.w2;

        self.seg[self.idx(r, c)] = x;

        loop {
            let mut c2 = c;
            while c2 != 0 {
                if (c2 & 1) == 0 {
                    c2 -= 1;
                }
                let mut t = c2;
                c2 >>= 1;

                let a1 = self.seg[self.idx(r, t)];
                t += 1;
                let a2 = self.seg[self.idx(r, t)];
                let a = if a1 > a2 { a2 } else { a1 };
                self.seg[self.idx(r, c2)] = a;
            }

            if r == 0 {
                break;
            }

            if (r & 1) == 0 {
                r -= 1;
            }
            let mut t = r;
            r >>= 1;

            let a1 = self.seg[self.idx(t, c)];
            t += 1;
            let a2 = self.seg[self.idx(t, c)];
            let a = if a1 > a2 { a2 } else { a1 };
            self.seg[self.idx(r, c)] = a;

            // keep looping while r >= 0 in C; here r is usize so loop ends at r==0 above.
            // Also note: `w2` is unused but kept to mirror structure; remove warning:
            let _ = w2;
        }
    }

    fn query2(&mut self, l: usize, r: usize, k: usize) -> i32 {
        // Mirrors C recursion exactly; uses self.c1/self.c2 and self.g.
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
        // Mirrors C recursion exactly; uses self.r1/self.r2 and sets self.g.
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
}

fn main() {
    // Read all input, parse as i32 like scanf("%d").
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

        // C loop condition: while (scanf(...) && R > 0)
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
            let r1: i32 = it.next().unwrap().parse().unwrap();
            let c1: i32 = it.next().unwrap().parse().unwrap();
            let r2: i32 = it.next().unwrap().parse().unwrap();
            let c2: i32 = it.next().unwrap().parse().unwrap();

            // C does: r2++, c2++; and uses half-open [r1,r2), [c1,c2)
            st.r1 = r1 as usize;
            st.c1 = c1 as usize;
            st.r2 = (r2 + 1) as usize;
            st.c2 = (c2 + 1) as usize;

            let ans = st.query(0, st.h, 0);
            out.push_str(&format!("{ans}\n"));
        }
    }

    print!("{out}");
}

use std::io::{self, Read};

const INF: i32 = 0x7fffffff;

struct Scanner {
    buf: Vec<u8>,
    idx: usize,
}
impl Scanner {
    fn new() -> Self {
        let mut input = Vec::new();
        io::stdin().read_to_end(&mut input).unwrap();
        Self { buf: input, idx: 0 }
    }
    #[inline]
    fn next_i32(&mut self) -> Option<i32> {
        while self.idx < self.buf.len() && self.buf[self.idx].is_ascii_whitespace() {
            self.idx += 1;
        }
        if self.idx >= self.buf.len() {
            return None;
        }
        let mut sign = 1i32;
        if self.buf[self.idx] == b'-' {
            sign = -1;
            self.idx += 1;
        }
        let mut x: i32 = 0;
        while self.idx < self.buf.len() {
            let b = self.buf[self.idx];
            if !b.is_ascii_digit() {
                break;
            }
            x = x * 10 + (b - b'0') as i32;
            self.idx += 1;
        }
        Some(x * sign)
    }
    #[inline]
    fn next_usize(&mut self) -> usize {
        self.next_i32().unwrap() as usize
    }
}

/// 2D segment tree as in the C code, but with O(logR*logC) build (not per-cell update).
struct Seg2D {
    h: usize,
    w: usize,
    h2: usize,
    w2: usize,
    seg: Vec<i32>, // flattened [h2][w2]
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
        // C memset(pool,0,...)
        let seg = vec![0i32; h2 * w2];
        Self { h, w, h2, w2, seg }
    }

    #[inline]
    fn idx(&self, rr: usize, cc: usize) -> usize {
        rr * self.w2 + cc
    }
    #[inline]
    fn get(&self, rr: usize, cc: usize) -> i32 {
        self.seg[self.idx(rr, cc)]
    }
    #[inline]
    fn set(&mut self, rr: usize, cc: usize, val: i32) {
        let i = self.idx(rr, cc);
        self.seg[i] = val;
    }

    fn build_from_grid(&mut self, r: usize, c: usize, grid: &[i32]) {
        // Place leaves for actual grid cells.
        let row_base = self.h - 1;
        let col_base = self.w - 1;
        for rr in 0..r {
            let seg_r = row_base + rr;
            let base = rr * c;
            for cc in 0..c {
                self.set(seg_r, col_base + cc, grid[base + cc]);
            }
        }

        // Build column trees for each row node bottom-up.
        for rr in 0..self.h2 {
            // internal column nodes: indices 0..(w-1) are internal, leaves start at w-1
            for cc in (0..(self.w - 1)).rev() {
                let left = 2 * cc + 1;
                let right = left + 1;
                let v = self.get(rr, left).min(self.get(rr, right));
                self.set(rr, cc, v);
            }
        }

        // Build row tree: for each internal row node, combine children for all column nodes.
        for rr in (0..(self.h - 1)).rev() {
            let left_r = 2 * rr + 1;
            let right_r = left_r + 1;
            let row_off = rr * self.w2;
            let left_off = left_r * self.w2;
            let right_off = right_r * self.w2;
            // combine across all column nodes (including leaves and internal)
            for cc in 0..self.w2 {
                let v = self.seg[left_off + cc].min(self.seg[right_off + cc]);
                self.seg[row_off + cc] = v;
            }
        }
    }

    #[inline]
    fn query(&self, r1: usize, c1: usize, r2_excl: usize, c2_excl: usize) -> i32 {
        self.query_rows(0, self.h, 0, r1, r2_excl, c1, c2_excl)
    }

    fn query_rows(
        &self,
        l: usize,
        r: usize,
        k: usize,
        r1: usize,
        r2: usize,
        c1: usize,
        c2: usize,
    ) -> i32 {
        if r2 <= l || r <= r1 {
            return INF;
        }
        if r1 <= l && r <= r2 {
            return self.query_cols(0, self.w, 0, k, c1, c2);
        }
        let m = (l + r) >> 1;
        let left = self.query_rows(l, m, 2 * k + 1, r1, r2, c1, c2);
        let right = self.query_rows(m, r, 2 * k + 2, r1, r2, c1, c2);
        left.min(right)
    }

    fn query_cols(
        &self,
        l: usize,
        r: usize,
        k: usize,
        row_node: usize,
        c1: usize,
        c2: usize,
    ) -> i32 {
        if c2 <= l || r <= c1 {
            return INF;
        }
        if c1 <= l && r <= c2 {
            return self.get(row_node, k);
        }
        let m = (l + r) >> 1;
        let left = self.query_cols(l, m, 2 * k + 1, row_node, c1, c2);
        let right = self.query_cols(m, r, 2 * k + 2, row_node, c1, c2);
        left.min(right)
    }
}

fn main() {
    let mut sc = Scanner::new();
    let mut out = String::new();

    loop {
        let r = match sc.next_i32() {
            Some(v) => v,
            None => break,
        };
        let c = sc.next_i32().unwrap();
        let q = sc.next_i32().unwrap();
        if r <= 0 {
            break;
        }

        let r_us = r as usize;
        let c_us = c as usize;
        let q_us = q as usize;

        let mut grid = vec![0i32; r_us * c_us];
        for i in 0..grid.len() {
            grid[i] = sc.next_i32().unwrap();
        }

        let mut seg = Seg2D::new(r_us, c_us);
        seg.build_from_grid(r_us, c_us, &grid);

        for _ in 0..q_us {
            let r1 = sc.next_usize();
            let c1 = sc.next_usize();
            let mut r2 = sc.next_usize();
            let mut c2 = sc.next_usize();
            // inclusive -> half-open, as in C
            r2 += 1;
            c2 += 1;

            let ans = seg.query(r1, c1, r2, c2);
            out.push_str(&ans.to_string());
            out.push('\n');
        }
    }

    print!("{out}");
}

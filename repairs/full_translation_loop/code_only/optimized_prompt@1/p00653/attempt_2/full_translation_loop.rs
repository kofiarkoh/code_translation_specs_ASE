use std::io::{self, BufRead};

const INF: i32 = 0x7fffffff;

struct SegmentTree {
    seg: Vec<Vec<i32>>,
    h: usize,
    w: usize,
    g: usize,
}

impl SegmentTree {
    fn new(r: usize, c: usize) -> Self {
        let mut h = 1;
        let mut w = 1;
        while h < r { h <<= 1; }
        while w < c { w <<= 1; }

        let h2 = (h << 1) - 1;
        let w2 = (w << 1) - 1;

        let seg = vec![vec![0; w2]; h2];

        SegmentTree { seg, h, w, g: 0 }
    }

    fn update(&mut self, r: usize, c: usize, x: i32) {
        let mut r = r + self.h - 1;
        let mut c = c + self.w - 1;
        self.seg[r][c] = x;

        while r > 0 {
            let mut c2 = c;
            while c2 > 0 {
                if c2 & 1 == 0 { c2 -= 1; }
                let t = c2;
                c2 >>= 1;
                let a = self.seg[r][t + 1].min(self.seg[r][t]);
                self.seg[r][c2] = a;
            }
            if r == 0 { break; }

            if r & 1 == 0 { r -= 1; }
            let t = r;
            r >>= 1;
            let a = self.seg[t + 1][c].min(self.seg[t][c]);
            self.seg[r][c] = a;
        }
    }

    fn query2(&self, l: usize, r: usize, k: usize, c1: usize, c2: usize) -> i32 {
        if c2 <= l || r <= c1 { return INF; }
        if c1 <= l && r <= c2 { return self.seg[self.g][k]; }

        let m = (l + r) >> 1;
        let lmin = self.query2(l, m, k << 1, c1, c2);
        let rmin = self.query2(m, r, (k << 1) + 1, c1, c2);
        lmin.min(rmin)
    }

    fn query(&mut self, l: usize, r: usize, k: usize, r1: usize, r2: usize) -> i32 {
        if r2 <= l || r <= r1 { return INF; }
        if r1 <= l && r <= r2 { 
            self.g = k; 
            return self.query2(0, self.w, 0, c1, c2); 
        }

        let m = (l + r) >> 1;
        let lmin = self.query(l, m, k << 1, r1, r2);
        let rmin = self.query(m, r, (k << 1) + 1, r1, r2);
        lmin.min(rmin)
    }
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    while let Some(line) = lines.next() {
        let line = line.unwrap();
        let mut parts = line.split_whitespace();
        let r: usize = parts.next().unwrap().parse().unwrap();
        let c: usize = parts.next().unwrap().parse().unwrap();
        let q: usize = parts.next().unwrap().parse().unwrap();

        if r <= 0 { break; }

        let mut seg_tree = SegmentTree::new(r, c);

        for r in 0..r {
            let line = lines.next().unwrap().unwrap();
            let values: Vec<i32> = line.split_whitespace().map(|x| x.parse().unwrap()).collect();
            for (c, &v) in values.iter().enumerate() {
                seg_tree.update(r, c, v);
            }
        }

        for _ in 0..q {
            let line = lines.next().unwrap().unwrap();
            let mut parts = line.split_whitespace();
            let r1: usize = parts.next().unwrap().parse().unwrap();
            let c1: usize = parts.next().unwrap().parse().unwrap();
            let r2: usize = parts.next().unwrap().parse().unwrap();
            let c2: usize = parts.next().unwrap().parse().unwrap();
            let result = seg_tree.query(0, seg_tree.h, 0, r1, r2 + 1);
            println!("{}", result);
        }
    }
}

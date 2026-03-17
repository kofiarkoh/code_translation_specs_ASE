// AOJ 2431: House Moving
// 2017.12.30 bal4u@uu

use std::io::{self, BufRead};

const MAX_SIZE: usize = 262145;

struct SegmentTree {
    seg: [i64; MAX_SIZE],
    sz: usize,
}

impl SegmentTree {
    fn new(n: usize) -> Self {
        let mut sz = 1;
        while sz < n {
            sz <<= 1;
        }
        SegmentTree {
            seg: vec![0; sz * 2].try_into().unwrap(), // Adjusted to allocate the correct size
            sz,
        }
    }

    // update kth value to v
    fn update(&mut self, mut k: usize, v: i64) {
        k += self.sz - 1;
        self.seg[k] = v;
        while k > 0 {
            k >>= 1;
            self.seg[k] = self.seg[2 * k + 1].max(self.seg[2 * k + 2]);
        }
    }

    // get max value in [a, b)
    fn query(&self, a: usize, b: usize, mut k: usize, l: usize, r: usize) -> i64 {
        if r <= a || b <= l {
            return 0;
        }
        if a <= l && r <= b {
            return self.seg[k];
        }
        k <<= 1;
        let m = (l + r) >> 1;
        let lmax = self.query(a, b, k + 1, l, m);
        let rmax = self.query(a, b, k + 2, m, r);
        lmax.max(rmax)
    }
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    let mut seg_tree = SegmentTree::new(n);

    let values: Vec<usize> = lines.next().unwrap().unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    for &x in &values {
        let query_result = seg_tree.query(0, x, 1, 0, seg_tree.sz);
        seg_tree.update(x, (x as i64) + query_result);
    }

    let ans = ((n as i64) * (n as i64 + 1) >> 1) - seg_tree.query(0, n, 1, 0, seg_tree.sz);
    println!("{}", ans);
}

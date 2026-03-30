// AOJ 2431: House Moving
// 2017.12.30 bal4u@uu

use std::io::{self, BufRead};

const MAX_SIZE: usize = 262145; // 2^18 = 262144

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
            seg: [0; MAX_SIZE],
            sz,
        }
    }

    // update kth value to v
    fn update(&mut self, mut k: usize, v: i64) {
        k += self.sz - 1;
        self.seg[k] = v;
        while k > 0 {
            if k & 1 == 0 {
                k -= 1;
            }
            let t = k;
            k >>= 1;
            let mut max_val = self.seg[t + 1];
            if self.seg[t] > max_val {
                max_val = self.seg[t];
            }
            self.seg[k] = max_val;
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
        if lmax < rmax {
            return rmax;
        }
        lmax
    }
}

fn in_value(p: &mut &str) -> i32 {
    let mut n = 0;
    while let Some(c) = p.chars().next() {
        if c < '0' {
            break;
        }
        n = 10 * n + (c as i32 - '0' as i32);
        *p = &p[1..]; // advance the pointer
    }
    *p = &p[1..]; // skip the next character (likely a space or newline)
    n
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines().map(|line| line.unwrap());

    let mut buf = lines.next().unwrap();
    let n = in_value(&mut buf.as_mut_str());

    let mut seg_tree = SegmentTree::new(n);
    buf = lines.next().unwrap();
    
    for _ in 0..n {
        let x = in_value(&mut buf.as_mut_str());
        seg_tree.update(x as usize, (x as i64) + seg_tree.query(1, x as usize, 0, 0, seg_tree.sz));
    }

    let ans = ((n as i64) * (n as i64 + 1) >> 1) - seg_tree.query(1, n + 1, 0, 0, seg_tree.sz);
    println!("{}", ans);
}

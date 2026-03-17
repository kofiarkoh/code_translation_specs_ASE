use std::io::{self, BufRead};

const MAX_SIZE: usize = 262145; // 2^18 = 262144

struct SegmentTree {
    seg: [i64; MAX_SIZE],
    sz: usize,
}

impl SegmentTree {
    fn new(size: usize) -> Self {
        let mut seg = [0; MAX_SIZE];
        let mut sz = 1;
        while sz < size {
            sz <<= 1;
        }
        SegmentTree { seg, sz }
    }

    // Update the kth value to v
    fn update(&mut self, k: usize, v: i64) {
        let mut k = k + self.sz - 1;
        self.seg[k] = v;
        while k > 0 {
            if k & 1 == 0 {
                k -= 1;
            }
            let t = k;
            k >>= 1;
            let left_value = self.seg[t + 1];
            let right_value = self.seg[t];
            self.seg[k] = left_value.max(right_value);
        }
    }

    // Get max value in [a, b)
    fn query(&self, a: usize, b: usize, k: usize, l: usize, r: usize) -> i64 {
        if r <= a || b <= l {
            return 0;
        }
        if a <= l && r <= b {
            return self.seg[k];
        }
        let m = (l + r) >> 1;
        let left_max = self.query(a, b, k << 1, l, m);
        let right_max = self.query(a, b, k << 1 | 1, m, r);
        left_max.max(right_max)
    }
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    let mut lines = reader.lines();
    let n: usize = lines.next().unwrap().unwrap().trim().parse().expect("Failed to parse n");

    let mut segment_tree = SegmentTree::new(n);
    let mut ans: i64;

    for _ in 0..n {
        let x: usize = lines.next().unwrap().unwrap().trim().parse().expect("Failed to parse x");
        if x >= n {
            panic!("Input x is out of bounds: {}", x);
        }
        let query_result = segment_tree.query(1, x + 1, 1, 0, segment_tree.sz);
        segment_tree.update(x, (x as i64) + query_result);
    }

    ans = ((n as i64) * (n as i64 + 1) >> 1) - segment_tree.query(1, n + 1, 1, 0, segment_tree.sz);
    println!("{}", ans);
}

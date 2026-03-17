// AOJ 2431: House Moving
// Translated from C to Rust

use std::io::{self, BufRead};

struct SegmentTree {
    seg: Vec<i64>,
    sz: usize,
}

impl SegmentTree {
    fn new(size: usize) -> Self {
        let sz = size.next_power_of_two(); // Ensure size is a power of two
        let seg = vec![0; 2 * sz]; // Initialize segment tree
        SegmentTree { seg, sz }
    }

    // Update kth value to v
    fn update(&mut self, k: usize, v: i64) {
        let mut k = k + self.sz - 1;
        self.seg[k] = v;
        while k > 0 {
            if k % 2 == 0 {
                k -= 1;
            }
            let t = k;
            k /= 2;
            let left_value = self.seg[t];
            let right_value = self.seg[t + 1];
            self.seg[k] = left_value.max(right_value);
        }
    }

    // Get max value in [a, b)
    fn query(&self, a: usize, b: usize, k: usize, l: usize, r: usize) -> i64 {
        if r <= a || b <= l {
            return 0; // Out of range
        }
        if a <= l && r <= b {
            return self.seg[k]; // Current segment is fully within range
        }
        let m = (l + r) / 2;
        let left_max = self.query(a, b, 2 * k + 1, l, m);
        let right_max = self.query(a, b, 2 * k + 2, m, r);
        left_max.max(right_max)
    }
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    // Read number of houses
    let mut lines = reader.lines();
    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    let mut segment_tree = SegmentTree::new(n);
    let mut total_cost = 0;

    // Read positions and calculate total cost
    let positions: Vec<usize> = lines
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    for &x in &positions {
        let current_cost = (x as i64) + segment_tree.query(1, x + 1, 0, 0, segment_tree.sz);
        segment_tree.update(x, current_cost);
    }

    total_cost = ((n as i64) * (n as i64 + 1)) >> 1 - segment_tree.query(1, n + 1, 0, 0, segment_tree.sz);
    println!("{}", total_cost);
}

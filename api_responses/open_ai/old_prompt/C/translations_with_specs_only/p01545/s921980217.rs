// House Moving Problem Implementation in Rust

use std::io::{self, BufRead};

struct SegmentTree {
    size: usize,
    tree: Vec<i32>,
}

impl SegmentTree {
    // Initialize the segment tree with a given size
    fn new(size: usize) -> Self {
        let tree = vec![0; 4 * size]; // Allocate space for the segment tree
        SegmentTree { size, tree }
    }

    // Update the segment tree with a new position
    fn update(&mut self, idx: usize, value: i32, node: usize, start: usize, end: usize) {
        if start == end {
            self.tree[node] = value; // Leaf node
        } else {
            let mid = (start + end) / 2;
            if idx <= mid {
                self.update(idx, value, 2 * node + 1, start, mid);
            } else {
                self.update(idx, value, 2 * node + 2, mid + 1, end);
            }
            self.tree[node] = self.tree[2 * node + 1].max(self.tree[2 * node + 2]); // Update the parent node
        }
    }

    // Query the maximum value in the range [l, r]
    fn query(&self, l: usize, r: usize, node: usize, start: usize, end: usize) -> i32 {
        if r < start || end < l {
            return 0; // Out of range
        }
        if l <= start && end <= r {
            return self.tree[node]; // Current segment is fully within the range
        }
        let mid = (start + end) / 2;
        let left_max = self.query(l, r, 2 * node + 1, start, mid);
        let right_max = self.query(l, r, 2 * node + 2, mid + 1, end);
        left_max.max(right_max) // Return the maximum of both segments
    }
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Read the number of houses
    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
    if n == 0 {
        // Undefined behavior for n = 0, but we handle it gracefully
        println!("0");
        return;
    }

    let mut positions = Vec::new();
    for _ in 0..n {
        let x: i32 = lines.next().unwrap().unwrap().parse().unwrap();
        positions.push(x);
    }

    // Initialize the segment tree
    let mut seg_tree = SegmentTree::new(n);

    // Update the segment tree with house positions
    for (i, &pos) in positions.iter().enumerate() {
        seg_tree.update(i, pos, 0, 0, n - 1);
    }

    // Calculate the total cost of moving houses
    let mut total_cost = 0;
    for i in 0..n {
        let max_pos = seg_tree.query(0, n - 1, 0, 0, n - 1);
        total_cost += (max_pos - positions[i]).abs(); // Cost is the distance moved
    }

    // Print the total cost
    println!("{}", total_cost);
}

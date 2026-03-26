use std::io::{self, BufRead};

struct SegmentTree {
    size: usize,
    data: Vec<i32>,
}

impl SegmentTree {
    fn new(size: usize) -> Self {
        let data = vec![0; size * 4]; // Allocate space for the segment tree
        SegmentTree { size, data }
    }

    fn update(&mut self, idx: usize, value: i32, node: usize, start: usize, end: usize) {
        if start == end {
            self.data[node] = value; // Leaf node
        } else {
            let mid = (start + end) / 2;
            if idx <= mid {
                self.update(idx, value, node * 2 + 1, start, mid);
            } else {
                self.update(idx, value, node * 2 + 2, mid + 1, end);
            }
            self.data[node] = self.data[node * 2 + 1].max(self.data[node * 2 + 2]); // Update the current node
        }
    }

    fn query(&self, l: usize, r: usize, node: usize, start: usize, end: usize) -> i32 {
        if r < start || end < l {
            return 0; // Out of range
        }
        if l <= start && end <= r {
            return self.data[node]; // Current segment is fully in range
        }
        let mid = (start + end) / 2;
        let left_max = self.query(l, r, node * 2 + 1, start, mid);
        let right_max = self.query(l, r, node * 2 + 2, mid + 1, end);
        left_max.max(right_max) // Return the maximum value in the range
    }
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Read number of houses, ensuring it is a valid integer
    let n: usize = match lines.next() {
        Some(Ok(line)) => line.parse().unwrap_or(0), // Default to 0 on parse error
        _ => 0,
    };

    if n == 0 {
        println!("0"); // If no houses, cost is 0
        return;
    }

    let mut positions = Vec::with_capacity(n);

    for _ in 0..n {
        if let Some(Ok(line)) = lines.next() {
            if let Ok(x) = line.parse::<i32>() {
                positions.push(x); // Read position of each house
            }
        }
    }

    let mut seg_tree = SegmentTree::new(n);
    let mut total_cost = 0;

    for (i, &pos) in positions.iter().enumerate() {
        seg_tree.update(i, pos, 0, 0, n - 1); // Update segment tree with house positions
        let max_pos = seg_tree.query(0, i, 0, 0, n - 1); // Query maximum position up to current house
        total_cost += (pos - max_pos).abs(); // Calculate cost based on distance moved
    }

    println!("{}", total_cost); // Output the total cost
}

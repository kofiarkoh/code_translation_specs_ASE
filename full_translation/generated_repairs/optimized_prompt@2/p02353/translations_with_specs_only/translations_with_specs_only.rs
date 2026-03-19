struct SegmentTree {
    size: usize,
    tree: Vec<i32>,
    lazy: Vec<i32>,
}

impl SegmentTree {
    fn new(size: usize) -> Self {
        let mut tree = vec![0; 4 * size];
        let lazy = vec![0; 4 * size];
        SegmentTree { size, tree, lazy }
    }

    fn update_range(&mut self, start: usize, end: usize, value: i32) {
        self.update_range_util(0, 0, self.size - 1, start, end, value);
    }

    fn update_range_util(&mut self, node: usize, node_start: usize, node_end: usize, range_start: usize, range_end: usize, value: i32) {
        // If there are pending updates, apply them
        if self.lazy[node] != 0 {
            self.tree[node] += self.lazy[node] * (node_end - node_start + 1) as i32;
            if node_start != node_end {
                self.lazy[node * 2 + 1] += self.lazy[node];
                self.lazy[node * 2 + 2] += self.lazy[node];
            }
            self.lazy[node] = 0;
        }

        // No overlap
        if node_end < range_start || node_start > range_end {
            return;
        }

        // Total overlap
        if node_start >= range_start && node_end <= range_end {
            self.tree[node] += value * (node_end - node_start + 1) as i32;
            if node_start != node_end {
                self.lazy[node * 2 + 1] += value;
                self.lazy[node * 2 + 2] += value;
            }
            return;
        }

        // Partial overlap
        let mid = (node_start + node_end) / 2;
        self.update_range_util(node * 2 + 1, node_start, mid, range_start, range_end, value);
        self.update_range_util(node * 2 + 2, mid + 1, node_end, range_start, range_end, value);
        self.tree[node] = self.tree[node * 2 + 1] + self.tree[node * 2 + 2];
    }

    fn query_range(&mut self, start: usize, end: usize) -> i32 {
        self.query_range_util(0, 0, self.size - 1, start, end)
    }

    fn query_range_util(&mut self, node: usize, node_start: usize, node_end: usize, range_start: usize, range_end: usize) -> i32 {
        // If there are pending updates, apply them
        if self.lazy[node] != 0 {
            self.tree[node] += self.lazy[node] * (node_end - node_start + 1) as i32;
            if node_start != node_end {
                self.lazy[node * 2 + 1] += self.lazy[node];
                self.lazy[node * 2 + 2] += self.lazy[node];
            }
            self.lazy[node] = 0;
        }

        // No overlap
        if node_end < range_start || node_start > range_end {
            return 0;
        }

        // Total overlap
        if node_start >= range_start && node_end <= range_end {
            return self.tree[node];
        }

        // Partial overlap
        let mid = (node_start + node_end) / 2;
        let left_sum = self.query_range_util(node * 2 + 1, node_start, mid, range_start, range_end);
        let right_sum = self.query_range_util(node * 2 + 2, mid + 1, node_end, range_start, range_end);
        left_sum + right_sum
    }
}

fn main() {
    use std::io::{self, BufRead};

    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Read number of elements and number of queries
    let first_line = lines.next().unwrap().unwrap();
    let mut parts = first_line.split_whitespace();
    let n: usize = parts.next().unwrap().parse().unwrap();
    let q: usize = parts.next().unwrap().parse().unwrap();

    let mut segment_tree = SegmentTree::new(n);

    for _ in 0..q {
        let query_line = lines.next().unwrap().unwrap();
        let mut query_parts = query_line.split_whitespace();
        let query_type: i32 = query_parts.next().unwrap().parse().unwrap();
        let start: usize = query_parts.next().unwrap().parse().unwrap();
        let end: usize = query_parts.next().unwrap().parse().unwrap();

        if query_type == 1 {
            let value: i32 = query_parts.next().unwrap().parse().unwrap();
            segment_tree.update_range(start, end, value);
        } else if query_type == 2 {
            let sum = segment_tree.query_range(start, end);
            println!("{}", sum);
        }
    }
}

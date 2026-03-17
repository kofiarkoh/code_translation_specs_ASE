struct SegmentTree {
    size: usize,
    tree: Vec<i64>, // Using i64 to handle large sums
    lazy: Vec<i64>, // Lazy propagation array
}

impl SegmentTree {
    fn new(size: usize) -> Self {
        let tree = vec![0; 4 * size]; // Allocate enough space for the segment tree
        let lazy = vec![0; 4 * size]; // Lazy array for updates
        SegmentTree { size, tree, lazy }
    }

    fn update_range(&mut self, start: usize, end: usize, value: i64) {
        self.update_range_util(0, 0, self.size - 1, start, end, value);
    }

    fn update_range_util(&mut self, node: usize, node_start: usize, node_end: usize, range_start: usize, range_end: usize, value: i64) {
        // If there are pending updates, apply them
        if self.lazy[node] != 0 {
            self.tree[node] += self.lazy[node] * (node_end - node_start + 1) as i64;
            if node_start != node_end {
                self.lazy[node * 2 + 1] += self.lazy[node];
                self.lazy[node * 2 + 2] += self.lazy[node];
            }
            self.lazy[node] = 0;
        }

        // Out of range
        if node_end < range_start || node_start > range_end {
            return;
        }

        // Current segment is fully in range
        if node_start >= range_start && node_end <= range_end {
            self.tree[node] += value * (node_end - node_start + 1) as i64;
            if node_start != node_end {
                self.lazy[node * 2 + 1] += value;
                self.lazy[node * 2 + 2] += value;
            }
            return;
        }

        // If not fully in range, recur for children
        let mid = (node_start + node_end) / 2;
        self.update_range_util(node * 2 + 1, node_start, mid, range_start, range_end, value);
        self.update_range_util(node * 2 + 2, mid + 1, node_end, range_start, range_end, value);
        self.tree[node] = self.tree[node * 2 + 1] + self.tree[node * 2 + 2]; // Update current node
    }

    fn get_sum(&mut self, start: usize, end: usize) -> i64 {
        self.get_sum_util(0, 0, self.size - 1, start, end)
    }

    fn get_sum_util(&mut self, node: usize, node_start: usize, node_end: usize, range_start: usize, range_end: usize) -> i64 {
        // If there are pending updates, apply them
        if self.lazy[node] != 0 {
            self.tree[node] += self.lazy[node] * (node_end - node_start + 1) as i64;
            if node_start != node_end {
                self.lazy[node * 2 + 1] += self.lazy[node];
                self.lazy[node * 2 + 2] += self.lazy[node];
            }
            self.lazy[node] = 0;
        }

        // Out of range
        if node_end < range_start || node_start > range_end {
            return 0; // Return 0 for out of range queries
        }

        // Current segment is fully in range
        if node_start >= range_start && node_end <= range_end {
            return self.tree[node];
        }

        // If not fully in range, recur for children
        let mid = (node_start + node_end) / 2;
        let left_sum = self.get_sum_util(node * 2 + 1, node_start, mid, range_start, range_end);
        let right_sum = self.get_sum_util(node * 2 + 2, mid + 1, node_end, range_start, range_end);
        left_sum + right_sum
    }
}

fn main() {
    let n: usize = 10; // Example number of elements
    let mut segment_tree = SegmentTree::new(n);

    // Example queries
    segment_tree.update_range(0, 4, 5); // Update range [0, 4] with value 5
    let sum = segment_tree.get_sum(0, 4); // Get sum of range [0, 4]
    println!("{}", sum); // Output the sum
}

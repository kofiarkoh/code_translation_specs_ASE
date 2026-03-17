// Segment Tree implementation for range updates and sum queries

const INIT_VAL: i64 = 0; // Initial value for lazy propagation

struct SegmentTree {
    size: usize,
    tree: Vec<i64>,
    lazy: Vec<i64>,
}

impl SegmentTree {
    // Create a new Segment Tree
    fn new(num: usize) -> Self {
        let size = num.next_power_of_two(); // Ensure size is a power of two
        SegmentTree {
            size,
            tree: vec![INIT_VAL; 2 * size],
            lazy: vec![INIT_VAL; 2 * size],
        }
    }

    // Update the range [start, end] with value x
    fn update(&mut self, start: usize, end: usize, x: i64) {
        self.update_range(1, 0, self.size - 1, start, end, x);
    }

    // Helper function for range update
    fn update_range(&mut self, node: usize, node_start: usize, node_end: usize, start: usize, end: usize, x: i64) {
        // If there are pending updates, apply them
        if self.lazy[node] != INIT_VAL {
            self.tree[node] += self.lazy[node] * (node_end - node_start + 1) as i64;
            if node_start != node_end {
                self.lazy[node * 2] += self.lazy[node];
                self.lazy[node * 2 + 1] += self.lazy[node];
            }
            self.lazy[node] = INIT_VAL; // Clear lazy value
        }

        // No overlap
        if node_end < start || node_start > end {
            return;
        }

        // Total overlap
        if node_start >= start && node_end <= end {
            self.tree[node] += x * (node_end - node_start + 1) as i64;
            if node_start != node_end {
                self.lazy[node * 2] += x;
                self.lazy[node * 2 + 1] += x;
            }
            return;
        }

        // Partial overlap
        let mid = (node_start + node_end) / 2;
        self.update_range(node * 2, node_start, mid, start, end, x);
        self.update_range(node * 2 + 1, mid + 1, node_end, start, end, x);
        self.tree[node] = self.tree[node * 2] + self.tree[node * 2 + 1]; // Update current node
    }

    // Query the sum in the range [start, end]
    fn query(&mut self, start: usize, end: usize) -> i64 {
        self.query_range(1, 0, self.size - 1, start, end)
    }

    // Helper function for range query
    fn query_range(&mut self, node: usize, node_start: usize, node_end: usize, start: usize, end: usize) -> i64 {
        // If there are pending updates, apply them
        if self.lazy[node] != INIT_VAL {
            self.tree[node] += self.lazy[node] * (node_end - node_start + 1) as i64;
            if node_start != node_end {
                self.lazy[node * 2] += self.lazy[node];
                self.lazy[node * 2 + 1] += self.lazy[node];
            }
            self.lazy[node] = INIT_VAL; // Clear lazy value
        }

        // No overlap
        if node_end < start || node_start > end {
            return 0; // Return 0 for sum queries outside the range
        }

        // Total overlap
        if node_start >= start && node_end <= end {
            return self.tree[node];
        }

        // Partial overlap
        let mid = (node_start + node_end) / 2;
        let left_sum = self.query_range(node * 2, node_start, mid, start, end);
        let right_sum = self.query_range(node * 2 + 1, mid + 1, node_end, start, end);
        left_sum + right_sum
    }
}

fn main() {
    // Example usage
    let num = 8; // Size of the array
    let mut segment_tree = SegmentTree::new(num);

    // Example operations
    segment_tree.update(0, 3, 5); // Update range [0, 3] with value 5
    segment_tree.update(4, 7, 10); // Update range [4, 7] with value 10

    // Query sums
    let sum1 = segment_tree.query(0, 3); // Query sum for range [0, 3]
    let sum2 = segment_tree.query(4, 7); // Query sum for range [4, 7]

    // Output results
    println!("{}", sum1); // Output the sum for range [0, 3]
    println!("{}", sum2); // Output the sum for range [4, 7]
}

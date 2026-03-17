// Import necessary modules
use std::cmp;

// Define a struct for the Segment Tree
struct SegmentTree {
    data: Vec<i32>,
    size: usize,
}

impl SegmentTree {
    // Create a new Segment Tree
    fn new(size: usize) -> Self {
        let data = vec![0; 4 * size]; // Allocate enough space for the segment tree
        SegmentTree { data, size }
    }

    // Build the segment tree from the input array
    fn build(&mut self, arr: &[i32], node: usize, start: usize, end: usize) {
        if start == end {
            self.data[node] = arr[start]; // Leaf node
        } else {
            let mid = (start + end) / 2;
            self.build(arr, 2 * node + 1, start, mid);
            self.build(arr, 2 * node + 2, mid + 1, end);
            self.data[node] = cmp::max(self.data[2 * node + 1], self.data[2 * node + 2]); // Internal node
        }
    }

    // Update the segment tree with a new value
    fn update(&mut self, idx: usize, value: i32, node: usize, start: usize, end: usize) {
        if start == end {
            self.data[node] = value; // Update leaf node
        } else {
            let mid = (start + end) / 2;
            if idx <= mid {
                self.update(idx, value, 2 * node + 1, start, mid);
            } else {
                self.update(idx, value, 2 * node + 2, mid + 1, end);
            }
            self.data[node] = cmp::max(self.data[2 * node + 1], self.data[2 * node + 2]); // Update internal node
        }
    }

    // Query the maximum value in a range
    fn query(&self, l: usize, r: usize, node: usize, start: usize, end: usize) -> i32 {
        if r < start || end < l {
            return 0; // Out of range
        }
        if l <= start && end <= r {
            return self.data[node]; // Current segment is fully in range
        }
        let mid = (start + end) / 2;
        let left_max = self.query(l, r, 2 * node + 1, start, mid);
        let right_max = self.query(l, r, 2 * node + 2, mid + 1, end);
        cmp::max(left_max, right_max) // Return the maximum of both sides
    }
}

// Main function to process input and compute the result
fn main() {
    let n: usize = 5; // Example input for number of elements
    let updates: Vec<i32> = vec![1, 3, 2, 5, 4]; // Example input for updates

    // Preconditions check
    assert!(n > 0, "n must be a positive integer");
    for &value in &updates {
        assert!(value >= 0, "All integers must be non-negative");
    }

    // Initialize the segment tree
    let mut segment_tree = SegmentTree::new(n);
    segment_tree.build(&updates, 0, 0, n - 1); // Build the segment tree

    // Perform updates (in this case, we assume updates are the same as initial values)
    for (i, &value) in updates.iter().enumerate() {
        segment_tree.update(i, value, 0, 0, n - 1);
    }

    // Query the maximum value in the range from 0 to n-1
    let max_value = segment_tree.query(0, n - 1, 0, 0, n - 1);

    // Calculate the sum of the first n natural numbers
    let sum_natural = n * (n + 1) / 2;

    // Compute the final result
    let result = sum_natural - max_value;

    // Output the result
    println!("{}", result);
}

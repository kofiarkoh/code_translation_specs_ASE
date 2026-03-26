struct SegmentTree {
    n: usize,
    data: Vec<i64>,
}

impl SegmentTree {
    // Initializes the segment tree with a given number of elements
    fn new(n: usize) -> Self {
        assert!(n.is_power_of_two(), "n must be a power of 2");
        let data = vec![0; 2 * n]; // Allocate space for the segment tree
        SegmentTree { n, data }
    }

    // Updates the value at index `i` with the value `t`
    fn update(&mut self, i: usize, t: i64) {
        assert!(i < self.n, "Index out of bounds for update");
        let mut idx = i + self.n; // Move to the leaf node
        self.data[idx] = t; // Update the value at the leaf

        // Update the tree upwards
        while idx > 1 {
            idx /= 2;
            self.data[idx] = self.data[2 * idx].max(self.data[2 * idx + 1]);
        }
    }

    // Queries the maximum value in the range [0, n)
    fn query(&self) -> i64 {
        self.data[1] // The root of the segment tree contains the maximum value
    }
}

// Example usage
fn main() {
    let n = 8; // Must be a power of 2
    let mut seg_tree = SegmentTree::new(n);
    
    // Example updates
    seg_tree.update(0, 5);
    seg_tree.update(1, 10);
    seg_tree.update(2, 3);
    seg_tree.update(3, 8);
    
    // Query the maximum value after updates
    let max_value = seg_tree.query();
    println!("{}", max_value); // Outputs the maximum value in the range [0, n)
}

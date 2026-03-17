struct SegmentTree {
    size: usize,
    data: Vec<i64>, // Using i64 to accommodate long long
}

impl SegmentTree {
    // Initializes the segment tree with a given number of elements
    fn new(n: usize) -> Self {
        // n must be a power of 2; we assume this is guaranteed by the caller
        let data = vec![0; 2 * n]; // Allocate space for the segment tree
        SegmentTree { size: n, data }
    }

    // Updates the value at index `i` with the value `t`
    fn update(&mut self, i: usize, t: i64) {
        // Set the value at the leaf node
        self.data[i + self.size] = t;

        // Update the segment tree upwards
        let mut idx = i + self.size;
        while idx > 1 {
            idx /= 2;
            self.data[idx] = self.data[2 * idx].max(self.data[2 * idx + 1]);
        }
    }

    // Calculates the maximum value in the range [0, n)
    fn query(&self) -> i64 {
        self.data[1] // The root of the segment tree contains the maximum value
    }
}

fn main() {
    // Example usage
    let n = 8; // Must be a power of 2
    let mut seg_tree = SegmentTree::new(n);

    // Simulating updates and queries
    seg_tree.update(0, 5);
    println!("{}", seg_tree.query()); // Outputs the maximum value after the update

    seg_tree.update(1, 10);
    println!("{}", seg_tree.query()); // Outputs the maximum value after the update

    seg_tree.update(2, 3);
    println!("{}", seg_tree.query()); // Outputs the maximum value after the update
}

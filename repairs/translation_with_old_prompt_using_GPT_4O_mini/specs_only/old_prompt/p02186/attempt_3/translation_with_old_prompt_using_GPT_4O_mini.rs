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

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let mut parts = input.split_whitespace();
    let n: usize = parts.next().unwrap().parse().unwrap();
    let q: usize = parts.next().unwrap().parse().unwrap();

    // Ensure n is a power of 2 before initializing the segment tree
    if !n.is_power_of_two() {
        panic!("n must be a power of 2");
    }

    let mut seg_tree = SegmentTree::new(n);
    
    // Read initial values
    for i in 0..n {
        let mut value = String::new();
        std::io::stdin().read_line(&mut value).unwrap();
        let val: i64 = value.trim().parse().unwrap();
        seg_tree.update(i, val);
    }

    // Process queries
    for _ in 0..q {
        let mut query = String::new();
        std::io::stdin().read_line(&mut query).unwrap();
        let parts: Vec<i64> = query.split_whitespace().map(|s| s.parse().unwrap()).collect();
        let index = parts[0] as usize - 1; // Convert to 0-based index
        let value = parts[1];

        // Update the segment tree
        seg_tree.update(index, value);

        // Query the maximum value in the range [0, n)
        let max_value = seg_tree.query();
        println!("{}", max_value); // Output the maximum value after each update
    }
}

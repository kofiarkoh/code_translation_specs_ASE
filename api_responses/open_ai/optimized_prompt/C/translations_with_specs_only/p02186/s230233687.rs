struct SegmentTree {
    data: Vec<i32>,
    max: Vec<i32>,
    lazy: Vec<i32>,
    size: usize,
}

impl SegmentTree {
    fn new(arr: &[i32]) -> Self {
        let n = arr.len();
        let size = n.next_power_of_two() * 2; // Size of the segment tree
        let mut data = vec![0; size];
        let mut max = vec![0; size];
        let mut lazy = vec![0; size];

        // Initialize the segment tree
        for (i, &value) in arr.iter().enumerate() {
            data[n + i] = value;
            max[n + i] = value;
        }
        for i in (1..n).rev() {
            max[i] = max[2 * i].max(max[2 * i + 1]);
        }

        SegmentTree { data, max, lazy, size }
    }

    fn update(&mut self, index: usize, value: i32) {
        // Update the value at the given index
        let n = self.data.len() / 2;
        let pos = n + index;

        // Set the new value
        self.data[pos] = value;
        self.max[pos] = value;

        // Update the segment tree upwards
        let mut i = pos;
        while i > 1 {
            i /= 2;
            self.max[i] = self.max[2 * i].max(self.max[2 * i + 1]);
        }
    }

    fn query_max(&self) -> i32 {
        // Return the maximum value in the segment tree
        self.max[1]
    }
}

fn main() {
    // Example usage
    let n = 5; // Number of elements
    let q = 3; // Number of queries
    let arr = vec![1, 3, 5, 7, 9]; // Initial array

    let mut segment_tree = SegmentTree::new(&arr);
    println!("{}", segment_tree.query_max()); // Output the maximum after initialization

    // Process queries
    let queries = vec![(1, 10), (3, 6), (0, 4)]; // (index, new_value)
    for (i, t) in queries {
        segment_tree.update(i, t);
        println!("{}", segment_tree.query_max()); // Output the maximum after each update
    }
}

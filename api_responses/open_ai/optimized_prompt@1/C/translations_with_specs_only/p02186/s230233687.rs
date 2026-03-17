struct SegmentTree {
    size: usize,
    tree: Vec<i32>,
    lazy: Vec<i32>,
}

impl SegmentTree {
    fn new(arr: &[i32]) -> Self {
        let n = arr.len();
        let size = 1 << (n.next_power_of_two().trailing_zeros());
        let mut tree = vec![i32::MIN; size * 2];
        let mut lazy = vec![0; size * 2];

        // Initialize the segment tree with the input array
        for (i, &value) in arr.iter().enumerate() {
            tree[size + i] = value;
        }
        for i in (1..size).rev() {
            tree[i] = tree[i * 2].max(tree[i * 2 + 1]);
        }

        SegmentTree { size, tree, lazy }
    }

    fn update_range(&mut self, idx: usize, value: i32) {
        let idx = idx + self.size; // Adjust index to the leaf position
        self.tree[idx] = value; // Update the value at the leaf
        // Recalculate the maximum values up the tree
        let mut i = idx;
        while i > 1 {
            i /= 2;
            self.tree[i] = self.tree[i * 2].max(self.tree[i * 2 + 1]);
        }
    }

    fn query_max(&self, left: usize, right: usize) -> i32 {
        let mut left = left + self.size; // Adjust to leaf position
        let mut right = right + self.size; // Adjust to leaf position
        let mut max_value = i32::MIN;

        while left <= right {
            if left % 2 == 1 {
                max_value = max_value.max(self.tree[left]);
                left += 1;
            }
            if right % 2 == 0 {
                max_value = max_value.max(self.tree[right]);
                right -= 1;
            }
            left /= 2;
            right /= 2;
        }
        max_value
    }
}

fn main() {
    let n = 5; // Example size of the array
    let q = 3; // Example number of queries
    let arr = vec![1, 3, 5, 7, 9]; // Example initialization array

    let mut segment_tree = SegmentTree::new(&arr);

    // Output the maximum value in the range [0, n-1]
    println!("{}", segment_tree.query_max(0, n - 1));

    // Example queries
    let queries = vec![(1, 10), (3, 6), (0, 4)]; // (index, new_value)
    for (i, t) in queries {
        segment_tree.update_range(i, t);
        // Output the maximum value in the range [0, n-1] after each update
        println!("{}", segment_tree.query_max(0, n - 1));
    }
}

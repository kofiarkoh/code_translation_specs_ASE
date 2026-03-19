struct SegmentTree {
    n: usize,
    data: Vec<i32>,
}

impl SegmentTree {
    fn new(arr: &[i32]) -> Self {
        let n = arr.len();
        let mut data = vec![0; 4 * n]; // Allocate enough space for the segment tree
        let mut tree = SegmentTree { n, data };
        tree.build(arr, 0, 0, n - 1);
        tree
    }

    fn build(&mut self, arr: &[i32], node: usize, start: usize, end: usize) {
        if start == end {
            self.data[node] = arr[start];
        } else {
            let mid = (start + end) / 2;
            self.build(arr, 2 * node + 1, start, mid);
            self.build(arr, 2 * node + 2, mid + 1, end);
            self.data[node] = self.data[2 * node + 1].max(self.data[2 * node + 2]);
        }
    }

    fn update(&mut self, idx: usize, value: i32) {
        self.update_range(0, 0, self.n - 1, idx, value);
    }

    fn update_range(&mut self, node: usize, start: usize, end: usize, idx: usize, value: i32) {
        if start == end {
            self.data[node] = value; // Update the leaf node
        } else {
            let mid = (start + end) / 2;
            if idx <= mid {
                self.update_range(2 * node + 1, start, mid, idx, value);
            } else {
                self.update_range(2 * node + 2, mid + 1, end, idx, value);
            }
            self.data[node] = self.data[2 * node + 1].max(self.data[2 * node + 2]);
        }
    }

    fn query(&self) -> i32 {
        self.query_range(0, 0, self.n - 1)
    }

    fn query_range(&self, node: usize, start: usize, end: usize) -> i32 {
        self.data[node] // Return the maximum value in the range
    }
}

fn main() {
    let n: usize = 5; // Example size
    let q: usize = 3; // Example number of queries
    let arr = vec![1, 2, 3, 4, 5]; // Example array

    let mut segment_tree = SegmentTree::new(&arr);
    println!("{}", segment_tree.query()); // Output the maximum value after initialization

    // Example queries
    let queries = vec![(1, 10), (3, 6), (4, 1)];
    for (i, t) in queries {
        segment_tree.update(i, t);
        println!("{}", segment_tree.query()); // Output the maximum value after each update
    }
}

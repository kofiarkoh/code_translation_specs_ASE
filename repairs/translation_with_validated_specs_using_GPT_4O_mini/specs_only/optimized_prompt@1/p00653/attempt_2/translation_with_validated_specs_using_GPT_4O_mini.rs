use std::cmp;

const INF: i32 = i32::MAX; // Predefined constant for invalid queries

struct SegmentTree2D {
    rows: usize,
    cols: usize,
    tree: Vec<Vec<i32>>,
}

impl SegmentTree2D {
    // Initialize the segment tree with given dimensions and values
    fn new(rows: usize, cols: usize, values: Vec<Vec<i32>>) -> Self {
        let tree = vec![vec![INF; cols * 4]; rows * 4]; // Removed mutability
        let mut segment_tree = SegmentTree2D { rows, cols, tree };
        segment_tree.build(0, 0, rows - 1, 0, 0, cols - 1, &values);
        segment_tree
    }

    // Build the segment tree from the initial grid values
    fn build(&mut self, x: usize, y: usize, x1: usize, y1: usize, x2: usize, y2: usize, values: &Vec<Vec<i32>>) {
        if x1 == x2 && y1 == y2 {
            self.tree[x][y] = values[x1][y1];
            return;
        }
        let mid_x = (x1 + x2) / 2;
        let mid_y = (y1 + y2) / 2;

        self.build(x * 2 + 1, y * 2 + 1, x1, y1, mid_x, mid_y, values);
        self.build(x * 2 + 1, y * 2 + 2, x1, mid_y + 1, mid_x, y2, values);
        self.build(x * 2 + 2, y * 2 + 1, mid_x + 1, y1, x2, mid_y, values);
        self.build(x * 2 + 2, y * 2 + 2, mid_x + 1, mid_y + 1, x2, y2, values);

        self.tree[x][y] = cmp::min(cmp::min(self.tree[x * 2 + 1][y * 2 + 1], self.tree[x * 2 + 1][y * 2 + 2]),
                                   cmp::min(self.tree[x * 2 + 2][y * 2 + 1], self.tree[x * 2 + 2][y * 2 + 2]));
    }

    // Update a specific cell in the grid and the segment tree
    fn update(&mut self, row: usize, col: usize, value: i32) {
        self.update_helper(0, 0, self.rows - 1, 0, 0, self.cols - 1, row, col, value);
    }

    fn update_helper(&mut self, x: usize, y: usize, x1: usize, y1: usize, x2: usize, y2: usize, row: usize, col: usize, value: i32) {
        if x1 == x2 && y1 == y2 {
            self.tree[x][y] = value;
            return;
        }
        let mid_x = (x1 + x2) / 2;
        let mid_y = (y1 + y2) / 2;

        if row <= mid_x && col <= mid_y {
            self.update_helper(x * 2 + 1, y * 2 + 1, x1, y1, mid_x, mid_y, row, col, value);
        } else if row <= mid_x {
            self.update_helper(x * 2 + 1, y * 2 + 2, x1, mid_y + 1, mid_x, y2, row, col, value);
        } else if col <= mid_y {
            self.update_helper(x * 2 + 2, y * 2 + 1, mid_x + 1, y1, x2, mid_y, row, col, value);
        } else {
            self.update_helper(x * 2 + 2, y * 2 + 2, mid_x + 1, mid_y + 1, x2, y2, row, col, value);
        }

        self.tree[x][y] = cmp::min(cmp::min(self.tree[x * 2 + 1][y * 2 + 1], self.tree[x * 2 + 1][y * 2 + 2]),
                                   cmp::min(self.tree[x * 2 + 2][y * 2 + 1], self.tree[x * 2 + 2][y * 2 + 2]));
    }

    // Query the minimum value in a specified rectangular subregion
    fn query(&self, x1: usize, y1: usize, x2: usize, y2: usize) -> i32 {
        if x1 > x2 || y1 > y2 || x2 >= self.rows || y2 >= self.cols {
            return INF; // Return INF for invalid queries
        }
        self.query_helper(0, 0, self.rows - 1, 0, 0, self.cols - 1, x1, y1, x2, y2)
    }

    fn query_helper(&self, x: usize, y: usize, x1: usize, y1: usize, x2: usize, y2: usize, qx1: usize, qy1: usize, qx2: usize, qy2: usize) -> i32 {
        if qx1 > x2 || qx2 < x1 || qy1 > y2 || qy2 < y1 {
            return INF; // Out of bounds
        }
        if qx1 <= x1 && qx2 >= x2 && qy1 <= y1 && qy2 >= y2 {
            return self.tree[x][y]; // Current segment is completely within the query range
        }
        let mid_x = (x1 + x2) / 2;
        let mid_y = (y1 + y2) / 2;

        let left_top = self.query_helper(x * 2 + 1, y * 2 + 1, x1, y1, mid_x, mid_y, qx1, qy1, qx2, qy2);
        let right_top = self.query_helper(x * 2 + 1, y * 2 + 2, x1, mid_y + 1, mid_x, y2, qx1, qy1, qx2, qy2);
        let left_bottom = self.query_helper(x * 2 + 2, y * 2 + 1, mid_x + 1, y1, x2, mid_y, qx1, qy1, qx2, qy2);
        let right_bottom = self.query_helper(x * 2 + 2, y * 2 + 2, mid_x + 1, mid_y + 1, x2, y2, qx1, qy1, qx2, qy2);

        cmp::min(cmp::min(left_top, right_top), cmp::min(left_bottom, right_bottom))
    }
}

// Entry point of the program
fn main() {
    // Example usage can be added here
}

// A 2D Segment Tree implementation for efficient range minimum queries and updates on a grid of integers.

struct SegmentTree2D {
    rows: usize,
    cols: usize,
    tree: Vec<Vec<i32>>,
    grid: Vec<Vec<i32>>,
}

impl SegmentTree2D {
    // Initialize the segment tree with given dimensions
    fn new(rows: usize, cols: usize) -> Self {
        let mut tree = vec![vec![i32::MAX; 4 * cols]; 4 * rows];
        let grid = vec![vec![0; cols]; rows];
        SegmentTree2D { rows, cols, tree, grid }
    }

    // Update the value at (row, col) in the grid and the segment tree
    fn update(&mut self, row: usize, col: usize, value: i32) {
        self.grid[row][col] = value;
        self.update_tree(0, 0, self.rows - 1, self.cols - 1, row, col, value);
    }

    // Recursive function to update the segment tree
    fn update_tree(&mut self, node_row: usize, node_col: usize, row_start: usize, row_end: usize, col_start: usize, col_end: usize, row: usize, col: usize, value: i32) {
        if row_start == row_end && col_start == col_end {
            self.tree[node_row][node_col] = value;
            return;
        }

        let mid_row = (row_start + row_end) / 2;
        let mid_col = (col_start + col_end) / 2;

        if row <= mid_row && col <= mid_col {
            self.update_tree(4 * node_row + 1, 4 * node_col + 1, row_start, mid_row, col_start, mid_col, row, col, value);
        } else if row <= mid_row {
            self.update_tree(4 * node_row + 1, 4 * node_col + 2, row_start, mid_row, mid_col + 1, col_end, row, col, value);
        } else if col <= mid_col {
            self.update_tree(4 * node_row + 2, 4 * node_col + 1, mid_row + 1, row_end, col_start, mid_col, row, col, value);
        } else {
            self.update_tree(4 * node_row + 2, 4 * node_col + 2, mid_row + 1, row_end, mid_col + 1, col_end, row, col, value);
        }

        self.tree[node_row][node_col] = self.query_tree(node_row, node_col, row_start, row_end, col_start, col_end);
    }

    // Query the minimum value in the specified rectangle
    fn query(&self, r1: usize, c1: usize, r2: usize, c2: usize) -> i32 {
        self.query_tree(0, 0, self.rows - 1, self.cols - 1, r1, r2 - 1, c1, c2 - 1)
    }

    // Recursive function to query the segment tree
    fn query_tree(&self, node_row: usize, node_col: usize, row_start: usize, row_end: usize, col_start: usize, col_end: usize, r1: usize, r2: usize, c1: usize, c2: usize) -> i32 {
        if r1 > row_end || r2 < row_start || c1 > col_end || c2 < col_start {
            return i32::MAX; // Out of range
        }
        if r1 <= row_start && r2 >= row_end && c1 <= col_start && c2 >= col_end {
            return self.tree[node_row][node_col]; // Current segment is fully within the query range
        }

        let mid_row = (row_start + row_end) / 2;
        let mid_col = (col_start + col_end) / 2;

        let left_top = self.query_tree(4 * node_row + 1, 4 * node_col + 1, row_start, mid_row, col_start, mid_col, r1, r2, c1, c2);
        let right_top = self.query_tree(4 * node_row + 1, 4 * node_col + 2, row_start, mid_row, mid_col + 1, col_end, r1, r2, c1, c2);
        let left_bottom = self.query_tree(4 * node_row + 2, 4 * node_col + 1, mid_row + 1, row_end, col_start, mid_col, r1, r2, c1, c2);
        let right_bottom = self.query_tree(4 * node_row + 2, 4 * node_col + 2, mid_row + 1, row_end, mid_col + 1, col_end, r1, r2, c1, c2);

        left_top.min(right_top).min(left_bottom).min(right_bottom) // Return the minimum of the four quadrants
    }
}

// Main function to demonstrate the usage of the SegmentTree2D
fn main() {
    let r = 3; // Number of rows
    let c = 3; // Number of columns
    let mut segment_tree = SegmentTree2D::new(r, c);

    // Example updates
    segment_tree.update(0, 0, 5);
    segment_tree.update(1, 1, 3);
    segment_tree.update(2, 2, 8);

    // Example queries
    let min_value = segment_tree.query(0, 0, 2, 2); // Query for minimum in the rectangle (0,0) to (2,2)
    println!("{}", min_value); // Output the result
}

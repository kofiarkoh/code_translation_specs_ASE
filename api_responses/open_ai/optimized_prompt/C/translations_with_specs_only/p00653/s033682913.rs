// A structure to represent a two-dimensional grid for efficient updates and queries.
struct Grid {
    data: Vec<Vec<i32>>, // The grid data stored as a vector of vectors.
    rows: usize,         // Number of rows in the grid.
    cols: usize,         // Number of columns in the grid.
}

impl Grid {
    // Creates a new Grid with the specified dimensions.
    // If dimensions are non-positive, the function will panic.
    fn new(rows: usize, cols: usize) -> Self {
        if rows == 0 || cols == 0 {
            panic!("Grid dimensions must be positive integers.");
        }
        Grid {
            data: vec![vec![i32::MAX; cols]; rows], // Initialize with maximum values.
            rows,
            cols,
        }
    }

    // Populates the grid with the provided values.
    // Assumes values are provided in a flat manner for the grid.
    fn populate(&mut self, values: &[i32]) {
        let mut index = 0;
        for r in 0..self.rows {
            for c in 0..self.cols {
                if index < values.len() {
                    self.data[r][c] = values[index];
                    index += 1;
                }
            }
        }
    }

    // Queries the minimum value in the specified subregion of the grid.
    // Returns a predefined constant if the query is out of bounds.
    fn query_min(&self, r1: usize, c1: usize, r2: usize, c2: usize) -> i32 {
        // Check for out-of-bounds queries.
        if r1 >= self.rows || c1 >= self.cols || r2 >= self.rows || c2 >= self.cols {
            return i32::MIN; // Predefined constant for invalid query.
        }

        // Ensure the corners are in the correct order.
        let (r1, r2) = (r1.min(r2), r1.max(r2));
        let (c1, c2) = (c1.min(c2), c1.max(c2));

        // Find the minimum value in the specified subregion.
        let mut min_value = i32::MAX;
        for r in r1..=r2 {
            for c in c1..=c2 {
                min_value = min_value.min(self.data[r][c]);
            }
        }
        min_value
    }
}

// Example usage of the Grid structure.
fn main() {
    let mut grid = Grid::new(3, 3); // Create a 3x3 grid.
    grid.populate(&[5, 3, 8, 1, 6, 7, 4, 2, 9]); // Populate the grid with values.

    // Query the minimum value in the subregion defined by corners (0, 0) and (1, 1).
    let min_value = grid.query_min(0, 0, 1, 1);
    println!("Minimum value in the specified subregion: {}", min_value);
}

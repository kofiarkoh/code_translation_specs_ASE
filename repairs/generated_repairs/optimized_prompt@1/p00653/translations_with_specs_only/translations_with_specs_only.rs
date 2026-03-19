// A struct to represent a 2D grid for efficient updates and queries.
struct Grid {
    data: Vec<Vec<i32>>, // 2D vector to store grid values
    rows: usize,         // Number of rows in the grid
    cols: usize,         // Number of columns in the grid
    min_constant: i32,   // Constant to return for out-of-bounds queries
}

impl Grid {
    // Constructor to create a new Grid with given dimensions and values
    fn new(rows: usize, cols: usize, values: Vec<i32>) -> Self {
        // Preconditions: R and C must be positive
        if rows == 0 || cols == 0 {
            panic!("Grid dimensions must be positive integers.");
        }
        
        // Initialize the grid with the provided values
        let mut data = vec![vec![0; cols]; rows];
        let mut index = 0;
        
        for r in 0..rows {
            for c in 0..cols {
                // Populate the grid with values
                data[r][c] = values[index];
                index += 1;
            }
        }
        
        // Define a constant for out-of-bounds queries
        let min_constant = i32::MAX; // Using max value as a predefined constant
        
        Grid { data, rows, cols, min_constant }
    }

    // Method to query the minimum value in a specified subregion
    fn query(&self, r1: isize, c1: isize, r2: isize, c2: isize) -> i32 {
        // Check for out-of-bounds queries
        if r1 < 0 || c1 < 0 || r2 >= self.rows as isize || c2 >= self.cols as isize {
            return self.min_constant; // Return predefined constant for out-of-bounds
        }

        // Initialize minimum value to the maximum possible integer
        let mut min_value = i32::MAX;

        // Iterate over the specified subregion to find the minimum value
        for r in r1.max(0) as usize..=r2.min((self.rows - 1) as isize) as usize {
            for c in c1.max(0) as usize..=c2.min((self.cols - 1) as isize) as usize {
                min_value = min_value.min(self.data[r][c]);
            }
        }

        min_value
    }
}

// Main function to demonstrate usage
fn main() {
    let grid = Grid::new(3, 3, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    let min_value = grid.query(0, 0, 1, 1); // Should return 1
    println!("Minimum value in the subregion: {}", min_value);
}

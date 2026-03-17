fn main() {
    // Read the dimensions of the grid
    let (m, n) = read_dimensions();
    
    // Ensure that M and N are within valid constraints
    if m <= 0 || n <= 0 || n > 106 {
        panic!("Invalid dimensions for the grid.");
    }

    // Initialize the grid
    let mut a = [[0; 106]; 106]; // Fixed size array as per specification
    let mut total_operations = 0;

    // Read the grid values
    for i in 0..m {
        for j in 0..n {
            a[i][j] = read_binary_value(); // Assume this function reads a binary value (0 or 1)
        }
    }

    // Process each row
    for i in 0..m {
        let mut left = 0;
        let mut right = n - 1;

        while left <= right {
            // Count operations for the left side
            if a[i][left] == 1 {
                total_operations += 1;
            }
            // Count operations for the right side
            if left != right && a[i][right] == 1 {
                total_operations += 1;
            }
            left += 1;
            right -= 1;
        }
    }

    // Output the total count of operations
    println!("{}", total_operations);
}

// Placeholder function to read dimensions
fn read_dimensions() -> (usize, usize) {
    // Implementation to read M and N from input
    // This is a stub; actual implementation will depend on input method
    (5, 5) // Example values
}

// Placeholder function to read a binary value
fn read_binary_value() -> i32 {
    // Implementation to read a binary value (0 or 1) from input
    // This is a stub; actual implementation will depend on input method
    1 // Example value
}

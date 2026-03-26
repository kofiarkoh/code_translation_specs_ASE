use std::io::{self, BufRead};

fn main() {
    // Read the dimensions of the grid
    let (m, n) = read_dimensions();
    
    // Ensure that M and N are within the specified constraints
    if m <= 0 || n <= 0 || n > 106 {
        panic!("Invalid dimensions for the grid.");
    }

    // Initialize the grid
    let mut a = [[0; 106]; 106]; // Fixed size array as per specification
    let mut total_operations = 0;

    // Read the grid values
    for i in 0..m {
        for j in 0..n {
            a[i][j] = read_binary_value(); // Read binary value (0 or 1)
        }
    }

    // Process each row
    for i in 0..m {
        let mut left = 0;
        let mut right = n - 1;

        while left <= right {
            // Count '1's from both ends towards the center
            if a[i][left] == 1 {
                total_operations += 1;
            }
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

// Function to read dimensions from standard input
fn read_dimensions() -> (usize, usize) {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();
    
    // Read the first line containing M and N
    if let Some(Ok(line)) = lines.next() {
        let parts: Vec<usize> = line.split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();
        if parts.len() == 2 {
            return (parts[0], parts[1]);
        }
    }
    panic!("Failed to read dimensions.");
}

// Function to read a binary value from standard input
fn read_binary_value() -> i32 {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();
    
    // Read the next line for a binary value
    if let Some(Ok(line)) = lines.next() {
        if let Ok(value) = line.trim().parse::<i32>() {
            if value == 0 || value == 1 {
                return value;
            }
        }
    }
    panic!("Failed to read a binary value.");
}

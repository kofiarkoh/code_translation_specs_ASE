fn main() {
    // Read dimensions of the grid
    let (rows, cols) = read_dimensions();
    
    // Initialize the grid
    let mut grid = vec![vec![0; cols]; rows];
    
    // Read the binary values into the grid
    read_binary_values(&mut grid);
    
    // Count transitions from 1s to 0s
    let transition_count = count_transitions(&grid);
    
    // Output the total count of transitions
    println!("{}", transition_count);
}

// Function to read dimensions of the grid
fn read_dimensions() -> (usize, usize) {
    // For simplicity, we assume valid input is provided
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let dims: Vec<usize> = input.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    (dims[0], dims[1])
}

// Function to read binary values into the grid
fn read_binary_values(grid: &mut Vec<Vec<u8>>) {
    for row in grid.iter_mut() {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let values: Vec<u8> = input.split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        row.copy_from_slice(&values);
    }
}

// Function to count transitions from 1s to 0s in the grid
fn count_transitions(grid: &[Vec<u8>]) -> usize {
    let mut count = 0;
    
    for row in grid {
        for j in 1..row.len() {
            // Count transitions from 1 to 0
            if row[j - 1] == 1 && row[j] == 0 {
                count += 1;
            }
        }
    }
    
    count
}

// Define a constant for the maximum number of blocks to place
const MAX_BLOCKS: usize = 1000; // Adjust as necessary based on the specification

fn main() {
    // Initialize a vector to hold counts for orientations 1 to 6
    let mut orientation_counts = [0; 6];

    // Read the number of blocks to place
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    let num_blocks: usize = input.trim().parse().expect("Invalid number of blocks");

    // Ensure the number of blocks does not exceed the maximum
    if num_blocks > MAX_BLOCKS {
        panic!("Number of blocks exceeds maximum limit");
    }

    // Process each block's orientations
    for _ in 0..num_blocks {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read line");
        let orientations: Vec<usize> = input
            .trim()
            .split_whitespace()
            .map(|s| s.parse().expect("Invalid orientation"))
            .collect();

        // Ensure we have exactly two orientations
        if orientations.len() != 2 {
            panic!("Expected two orientations");
        }

        // Increment the counts for the top and front orientations
        for &orientation in &orientations {
            if orientation >= 1 && orientation <= 6 {
                orientation_counts[orientation - 1] += 1; // Adjust for 0-based index
            }
            // No error handling for out-of-range values as per specification
        }
    }

    // Output the counts of each orientation as space-separated integers
    println!(
        "{} {} {} {} {} {}",
        orientation_counts[0],
        orientation_counts[1],
        orientation_counts[2],
        orientation_counts[3],
        orientation_counts[4],
        orientation_counts[5]
    );
}

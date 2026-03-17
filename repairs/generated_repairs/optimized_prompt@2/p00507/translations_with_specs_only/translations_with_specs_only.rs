use std::collections::HashSet;
use std::convert::TryInto;

fn main() {
    // Read the number of integers
    let n: usize = read_input().try_into().unwrap(); // Assume read_input() reads a valid positive integer

    // Ensure N is at least 2 for valid concatenation
    if n < 2 {
        return; // Behavior is not defined for N < 2
    }

    // Read the integers into a vector
    let mut integers: Vec<i32> = Vec::new();
    for _ in 0..n {
        integers.push(read_input()); // Assume read_input() reads valid integers
    }

    // Use a HashSet to store unique concatenations
    let mut unique_concats: HashSet<String> = HashSet::new();

    // Generate unique concatenations of distinct pairs
    for i in 0..n {
        for j in 0..n {
            if i != j {
                // Concatenate integers as strings
                let concat1 = format!("{}{}", integers[i], integers[j]);
                unique_concats.insert(concat1);
            }
        }
    }

    // Collect unique concatenations into a vector and sort them
    let mut sorted_concats: Vec<String> = unique_concats.into_iter().collect();
    sorted_concats.sort();

    // Check if we have at least three unique concatenations
    if sorted_concats.len() < 3 {
        return; // Behavior is not defined if fewer than three unique concatenations exist
    }

    // Output the third smallest unique concatenation
    println!("{}", sorted_concats[2]);
}

// Placeholder function to simulate input reading
fn read_input() -> i32 {
    // This function should be implemented to read input from the user or a file
    // For the sake of this example, we will return a dummy value
    0 // Replace with actual input reading logic
}

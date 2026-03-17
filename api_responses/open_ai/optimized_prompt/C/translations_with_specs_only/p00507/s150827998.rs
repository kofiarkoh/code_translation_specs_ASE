use std::collections::HashSet;

fn main() {
    let n: usize = read_input(); // Read the number of integers
    let numbers: Vec<i32> = read_numbers(n); // Read the integers

    if n < 2 {
        // If there are fewer than 2 integers, we cannot form any concatenation
        return; // Behavior is not defined, so we exit
    }

    let mut unique_concats = HashSet::new();

    // Generate unique concatenations of distinct pairs
    for i in 0..n {
        for j in 0..n {
            if i != j {
                // Concatenate numbers[i] and numbers[j] as strings
                let concat1 = format!("{}{}", numbers[i], numbers[j]);
                unique_concats.insert(concat1);
            }
        }
    }

    // Collect unique concatenations into a vector and sort them
    let mut sorted_concats: Vec<String> = unique_concats.into_iter().collect();
    sorted_concats.sort();

    // Check if we have at least three unique concatenations
    if sorted_concats.len() < 3 {
        // Behavior is not defined if fewer than three unique concatenations exist
        return; // We exit as we cannot provide a valid output
    }

    // Output the third smallest unique concatenation
    println!("{}", sorted_concats[2]);
}

// Function to read the number of integers from input
fn read_input() -> usize {
    // Placeholder for actual input reading logic
    // This should read from standard input and parse it as usize
    5 // Example value for demonstration
}

// Function to read a list of integers from input
fn read_numbers(n: usize) -> Vec<i32> {
    // Placeholder for actual input reading logic
    // This should read from standard input and parse it into a Vec<i32>
    vec![1, 2, 3, 4, 5] // Example values for demonstration
}

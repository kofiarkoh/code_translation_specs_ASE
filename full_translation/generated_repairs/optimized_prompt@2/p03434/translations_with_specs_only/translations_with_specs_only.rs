use std::convert::TryInto;

fn main() {
    // Read the number of elements in the sequence
    let n: usize = read_input().try_into().unwrap(); // Convert i32 to usize
    let mut sequence: Vec<i32> = Vec::with_capacity(n);

    // Read the sequence of integers
    for _ in 0..n {
        sequence.push(read_input()); // Assuming a function to read input
    }

    // Sort the sequence in non-decreasing order
    sequence.sort();

    // Initialize sums for even and odd indices
    let mut sum_even = 0;
    let mut sum_odd = 0;

    // Compute the sums based on sorted sequence
    for (index, &value) in sequence.iter().enumerate() {
        if index % 2 == 0 {
            sum_even += value; // Sum for even indices
        } else {
            sum_odd += value; // Sum for odd indices
        }
    }

    // Compute the difference between the sums
    let difference = sum_even - sum_odd;

    // Output the result
    println!("{}", difference); // Assuming a function to output the result
}

// Placeholder function to read input, replace with actual implementation
fn read_input() -> i32 {
    // Implementation to read an integer from input
    // This is a stub and should be replaced with actual input handling
    0 // Placeholder return value
}

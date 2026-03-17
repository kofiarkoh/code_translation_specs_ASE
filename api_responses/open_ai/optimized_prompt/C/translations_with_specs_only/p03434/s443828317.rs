fn main() {
    // Read the number of elements
    let n: usize = read_input(); // Assume read_input() is defined to read an integer from input

    // Preconditions check
    if n == 0 {
        // Undefined behavior for N <= 0, we can choose to panic here
        panic!("N must be a positive integer.");
    }

    // Read the sequence of integers
    let mut sequence: Vec<i32> = read_sequence(n); // Assume read_sequence() is defined to read n integers

    // Sort the sequence in non-decreasing order
    sequence.sort();

    // Calculate the midpoint for splitting the sequence
    let mid = n / 2;

    // Compute the sum of the smallest half
    let sum_smallest_half: i32 = sequence.iter().take(mid).sum();

    // Compute the sum of the largest half
    let sum_largest_half: i32 = sequence.iter().skip(mid).sum();

    // Calculate the difference
    let difference = sum_largest_half - sum_smallest_half;

    // Output the result
    println!("{}", difference);
}

// Placeholder function to read an integer from input
fn read_input() -> usize {
    // Implementation would go here
    unimplemented!()
}

// Placeholder function to read a sequence of integers
fn read_sequence(n: usize) -> Vec<i32> {
    // Implementation would go here
    unimplemented!()
}

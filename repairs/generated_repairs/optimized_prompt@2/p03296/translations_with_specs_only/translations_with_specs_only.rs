fn main() {
    use std::io::{self, BufRead};

    // Read the first line of input, which contains the integer N
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    // Read N
    let n: usize = match lines.next() {
        Some(Ok(line)) => line.trim().parse().expect("Input must be a positive integer"),
        _ => panic!("Failed to read the first line"),
    };

    // Ensure N is within the specified limit
    assert!(n > 0 && n <= 101, "N must be a positive integer and at most 101");

    // Initialize a vector to store the sequence of integers
    let mut sequence = Vec::with_capacity(n);

    // Read the N integers into the sequence
    for _ in 0..n {
        let number: i32 = match lines.next() {
            Some(Ok(line)) => line.trim().parse().expect("Input must be an integer"),
            _ => panic!("Failed to read an integer"),
        };
        sequence.push(number);
    }

    // Initialize the count of consecutive equal pairs
    let mut count = 0;

    // Iterate through the sequence to count pairs of consecutive equal integers
    for i in 1..sequence.len() {
        if sequence[i] == sequence[i - 1] {
            count += 1; // Increment count if the current and previous integers are equal
        }
    }

    // Output the count of pairs
    println!("{}", count);
}

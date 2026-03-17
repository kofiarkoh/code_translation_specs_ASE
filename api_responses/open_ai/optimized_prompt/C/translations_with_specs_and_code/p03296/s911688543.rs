use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    
    // Read the number of integers N
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: usize = input.trim().parse().expect("Input must be a positive integer");

    // Preconditions check
    if n < 1 || n > 101 {
        // Behavior is not defined for N < 1 or N > 101, but we assume valid input as per spec
        return;
    }

    let mut a = vec![0; n]; // Create a vector to hold the integers

    // Read N integers
    for i in 0..n {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        a[i] = input.trim().parse().expect("Input must be an integer");
    }

    let mut count = 0;
    let mut i = 1;

    // Count pairs of consecutive equal integers
    while i < n {
        if a[i] == a[i - 1] {
            count += 1;
            i += 1; // Skip the next integer as it's part of the current pair
        }
        i += 1; // Move to the next integer
    }

    // Output the count
    println!("{}", count);
    io::stdout().flush().expect("Failed to flush stdout");
}

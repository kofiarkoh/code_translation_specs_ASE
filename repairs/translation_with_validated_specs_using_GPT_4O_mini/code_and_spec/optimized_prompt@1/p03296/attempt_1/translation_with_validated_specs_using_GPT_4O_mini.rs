use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: usize = input.trim().parse().expect("Input must be a positive integer");

    // Ensure the input size does not exceed the specified limit
    if n == 0 || n > 100 {
        eprintln!("N must be a positive integer and less than or equal to 100.");
        return;
    }

    let mut a = vec![0; n + 1]; // Create a vector to hold the integers, 1-indexed
    for i in 1..=n {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        a[i] = input.trim().parse().expect("Input must be an integer");
    }

    let mut count = 0;
    let mut i = 2; // Start from the second index
    while i <= n {
        if a[i] == a[i - 1] {
            count += 1; // Found a pair
            i += 1; // Skip the next integer
        }
        i += 1; // Move to the next integer
    }

    // Output the count
    println!("{}", count);
    io::stdout().flush().expect("Failed to flush stdout");
}

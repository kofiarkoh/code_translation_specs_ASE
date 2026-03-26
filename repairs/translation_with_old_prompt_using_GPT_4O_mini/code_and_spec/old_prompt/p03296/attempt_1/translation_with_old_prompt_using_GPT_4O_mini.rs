use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();

    // Read the number of integers
    let mut lines = handle.lines();
    let n: usize = lines.next().unwrap().unwrap().trim().parse().expect("Invalid input for N");

    // Ensure N is within the valid range
    if n < 1 || n > 100 {
        eprintln!("N must be between 1 and 100");
        return;
    }

    let mut a = vec![0; 101]; // Create a vector to hold up to 101 integers

    // Read the integers into the array
    for i in 1..=n {
        a[i] = lines.next().unwrap().unwrap().trim().parse().expect("Invalid integer input");
    }

    let mut count = 0;
    let mut i = 2;

    // Count consecutive duplicates
    while i <= n {
        if a[i] == a[i - 1] {
            count += 1;
            i += 1; // Skip the next element as it's part of the duplicate
        }
        i += 1; // Move to the next element
    }

    // Print the result
    println!("{}", count);
}

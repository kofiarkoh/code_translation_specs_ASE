fn main() {
    use std::io::{self, BufRead};

    // Read the number of integers from standard input
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    // Read the first line for N
    let n: usize = lines.next().unwrap().unwrap().trim().parse().expect("Invalid input for N");

    // Ensure N is within the valid range
    if n < 1 || n > 100 {
        panic!("N must be a positive integer between 1 and 100");
    }

    // Initialize an array to hold the integers
    let mut a: Vec<i32> = Vec::with_capacity(n);

    // Read the next N integers into the array
    for _ in 0..n {
        let value: i32 = lines.next().unwrap().unwrap().trim().parse().expect("Invalid integer input");
        a.push(value);
    }

    // Count consecutive duplicates
    let mut count = 0;
    for i in 1..n {
        if a[i] == a[i - 1] {
            count += 1; // Increment count for each pair of consecutive duplicates
        }
    }

    // Print the count of consecutive duplicates
    println!("{}", count);
}

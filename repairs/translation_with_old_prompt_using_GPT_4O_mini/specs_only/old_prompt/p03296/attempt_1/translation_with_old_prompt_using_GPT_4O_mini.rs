use std::io::{self, BufRead};

fn main() {
    // Read the number of integers (N)
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    // Read the first line for N
    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();

    // Initialize the array to hold the integers
    let mut a: [i32; 101] = [0; 101];

    // Read the integers into the array
    for i in 0..n {
        a[i] = lines.next().unwrap().unwrap().parse().unwrap();
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

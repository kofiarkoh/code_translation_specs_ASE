use std::io::{self, BufRead};

fn main() {
    // Read input from standard input
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Read the first line to get N
    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();

    // Initialize the result variable
    let mut ans: i64 = 0;

    // Compute the initial sum based on N
    // The initial sum should be the sum of the first N natural numbers
    ans += (n * (n + 1) / 2) as i64; // Sum of first N natural numbers

    // Process N-1 pairs of integers
    for _ in 0..(n - 1) {
        let line = lines.next().unwrap().unwrap();
        let mut parts = line.split_whitespace();
        let u: usize = parts.next().unwrap().parse().unwrap();
        let w: usize = parts.next().unwrap().parse().unwrap();

        // Adjust the answer based on the pairs (u, w)
        ans += (u as i64) * (w as i64);
    }

    // Print the result
    println!("{}", ans);
}

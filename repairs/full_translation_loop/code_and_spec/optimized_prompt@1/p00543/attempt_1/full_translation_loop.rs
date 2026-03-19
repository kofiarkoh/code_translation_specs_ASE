use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read n and m
    let first_line = lines.next().unwrap().unwrap();
    let mut parts = first_line.split_whitespace();
    let n: usize = parts.next().unwrap().parse().unwrap();
    let m: usize = parts.next().unwrap().parse().unwrap();

    // Preconditions: n and m must be positive integers
    assert!(n > 0 && m > 0, "n and m must be positive integers");

    // Read the array of integers
    let mut a = Vec::with_capacity(n);
    for _ in 0..n {
        let line = lines.next().unwrap().unwrap();
        let value: i32 = line.parse().unwrap();
        a.push(value);
    }

    // Ensure we have exactly n integers
    assert!(a.len() == n, "The number of integers provided must match n");

    // Sorting based on the remainder when divided by each integer from 1 to m
    for k in 1..=m {
        for i in 0..n - 1 {
            for j in i + 1..n {
                if a[i] % k > a[j] % k {
                    a.swap(i, j);
                }
            }
        }
    }

    // Output the sorted array, each on a new line
    for &value in &a {
        println!("{}", value);
    }
}

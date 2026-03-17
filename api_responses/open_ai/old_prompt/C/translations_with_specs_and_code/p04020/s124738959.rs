use std::io::{self, BufRead};

fn main() {
    // Read input from stdin
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Read the number of integers N
    let n: usize = lines.next().unwrap().unwrap().trim().parse().expect("Invalid input for N");

    // Ensure N is within the valid range
    assert!(n >= 1 && n <= 100_000, "N must be between 1 and 100,000");

    // Initialize the array A with size N + 1 for 1-based indexing
    let mut a: Vec<i64> = vec![0; n + 1];

    // Read the N long long integers into the array A
    for i in 1..=n {
        a[i] = lines.next().unwrap().unwrap().trim().parse().expect("Invalid input for A[i]");
    }

    let mut ans: i64 = 0;

    // Compute the sum based on the specified logic
    for i in 1..n {
        ans += a[i] / 2;
        if a[i + 1] > 1 && a[i] % 2 == 1 {
            a[i + 1] -= 1;
            ans += 1;
        }
    }
    ans += a[n] / 2;

    // Print the result
    println!("{}", ans);
}

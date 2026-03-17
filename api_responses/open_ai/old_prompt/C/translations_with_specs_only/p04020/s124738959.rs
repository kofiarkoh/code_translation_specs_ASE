fn main() {
    use std::io::{self, BufRead};

    // Read input from standard input
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Read the first line to get N
    let n: usize = lines.next().unwrap().unwrap().trim().parse().expect("Invalid input for N");

    // Ensure N is within the specified range
    assert!(n >= 1 && n <= 100_000, "N must be between 1 and 100,000");

    // Initialize the array A with size N + 1 for 1-based indexing
    let mut a: Vec<i64> = vec![0; n + 1];

    // Read the next N long long integers into the array A
    for i in 1..=n {
        a[i] = lines.next().unwrap().unwrap().trim().parse().expect("Invalid input for A[i]");
    }

    // Variable to hold the computed sum
    let mut ans: i64 = 0;

    // Process the array A according to the specification
    for i in 1..=n {
        // Halve the current element
        ans += a[i] / 2;

        // If the current element is odd, adjust the next element
        if a[i] % 2 != 0 {
            if i < n { // Ensure we don't go out of bounds
                a[i + 1] += 1;
            }
        }
    }

    // Output the computed sum
    println!("{}", ans);
}

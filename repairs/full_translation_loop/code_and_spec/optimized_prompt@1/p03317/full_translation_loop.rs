use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the first line containing n and k
    let first_line = lines.next().unwrap().unwrap();
    let mut inputs = first_line.split_whitespace();
    let n: i64 = inputs.next().unwrap().parse().expect("Invalid input for n");
    let k: i64 = inputs.next().unwrap().parse().expect("Invalid input for k");

    // Preconditions check
    assert!(n > 0, "n must be a positive integer");
    assert!(k > 1, "k must be a positive integer greater than one");

    // Read the sequence of integers
    let mut a = Vec::with_capacity(n as usize);
    for _ in 0..n {
        let value: i64 = lines.next().unwrap().unwrap().trim().parse().expect("Invalid input for sequence value");
        a.push(value);
    }

    // Calculate the minimum number of groups
    let mut t = (n - k) / (k - 1) + 1;
    if k + (t - 1) * (k - 1) < n {
        t += 1;
    }

    // Output the result
    println!("{}", t);
}
``` 

### Changes Made:
1. **Input Handling**: Ensure that the input values are trimmed before parsing to avoid issues with leading or trailing whitespace.
2. **Error Messages**: The error messages are kept for clarity, but ensure that the input format is strictly followed to avoid runtime errors.

If the issue persists, please check the input format in the provided input file to ensure it matches the expected format.

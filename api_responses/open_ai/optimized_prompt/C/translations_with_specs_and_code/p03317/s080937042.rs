use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    // Read the first line containing n and k
    let first_line = lines.next().unwrap().unwrap();
    let mut inputs = first_line.split_whitespace();
    let n: i64 = inputs.next().unwrap().parse().unwrap();
    let k: i64 = inputs.next().unwrap().parse().unwrap();

    // Preconditions check
    assert!(n > 0, "The number of elements must be positive.");
    assert!(k > 1, "The grouping parameter must be greater than one.");

    // Read the sequence of integers
    let mut a = Vec::with_capacity(n as usize);
    for _ in 0..n {
        let value: i64 = lines.next().unwrap().unwrap().parse().unwrap();
        a.push(value);
    }

    // Calculate the minimum number of groups required
    let mut t = (n - k) / (k - 1) + 1;
    if k + ((t - 1) * (k - 1)) < n {
        t += 1;
    }

    // Output the result
    println!("{}", t);
}

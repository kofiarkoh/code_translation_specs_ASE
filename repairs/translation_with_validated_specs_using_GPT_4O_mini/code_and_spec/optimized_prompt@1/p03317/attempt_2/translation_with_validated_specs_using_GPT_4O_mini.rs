use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Read n and k
    let first_line = lines.next().unwrap().unwrap();
    let mut inputs = first_line.split_whitespace();
    let n: i64 = inputs.next().unwrap().parse().expect("Failed to parse n");
    let k: i64 = inputs.next().unwrap().parse().expect("Failed to parse k");

    // Preconditions check
    if n <= 0 || k <= 1 {
        panic!("Invalid input: n must be positive and k must be greater than 1.");
    }

    // Read the sequence of integers
    let mut a = Vec::with_capacity(n as usize);
    for _ in 0..n {
        let line = lines.next().unwrap().unwrap();
        let value: i64 = line.parse().expect("Failed to parse integer in sequence");
        a.push(value);
    }

    // Calculate the number of groups
    let mut t = (n - k) / (k - 1) + 1;
    if k + ((t - 1) * (k - 1)) < n {
        t += 1;
    }

    // Output the result
    println!("{}", t);
}

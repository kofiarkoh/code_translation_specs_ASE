use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut input = stdin.lock().lines();

    // Read n and k
    let first_line = input.next().unwrap().unwrap();
    let mut nums = first_line.split_whitespace()
        .map(|s| s.parse::<i64>().unwrap());
    let n = nums.next().unwrap();
    let k = nums.next().unwrap();

    // Read the array a
    let mut a = Vec::with_capacity(n as usize);
    for _ in 0..n {
        let line = input.next().unwrap().unwrap();
        a.push(line.parse::<i64>().unwrap());
    }

    // Calculate t
    let mut t = (n - k) / (k - 1) + 1;
    if k + ((t - 1) * (k - 1)) < n {
        t += 1;
    }

    // Print the result
    println!("{}", t);
}

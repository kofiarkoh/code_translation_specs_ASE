use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    // Read n and k
    let first_line = lines.next().unwrap().unwrap();
    let mut nums = first_line.split_whitespace().map(|s| s.parse::<i64>());
    let n = nums.next().unwrap().expect("Failed to parse n");
    let k = nums.next().unwrap().expect("Failed to parse k");

    // Read array a
    let mut a = Vec::with_capacity(n as usize);
    for _ in 0..n {
        let line = lines.next().unwrap().unwrap();
        a.push(line.parse::<i64>().expect("Failed to parse array element"));
    }

    // Calculate t
    let mut t = (n - k) / (k - 1) + 1;
    if k + ((t - 1) * (k - 1)) < n {
        t += 1;
    }

    // Print result
    println!("{}", t);
}

use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read n and m
    let first_line = lines.next().unwrap().unwrap();
    let mut nums = first_line.split_whitespace().map(|s| s.parse::<i32>().unwrap());
    let n = nums.next().unwrap();
    let m = nums.next().unwrap();

    // Create vectors for x and y
    let mut x = vec![0; (n + 1) as usize];
    let mut y = vec![0; (m + 1) as usize];

    // Read x values
    for i in 1..=n {
        let line = lines.next().unwrap().unwrap();
        x[i as usize] = line.parse::<i64>().unwrap();
    }

    // Read y values
    for i in 1..=m {
        let line = lines.next().unwrap().unwrap();
        y[i as usize] = line.parse::<i64>().unwrap();
    }

    let mut t: i64 = 0;

    // Calculate t
    for i in 1..=n {
        t = (t + (i - 1) as i64 * x[i as usize] - (n - i) as i64 * x[i as usize]) % 1_000_000_007;
    }

    let mut k: i64 = 0;

    // Calculate k
    for i in 1..=m {
        k = (k + (i - 1) as i64 * y[i as usize] - (m - i) as i64 * y[i as usize]) % 1_000_000_007;
    }

    // Ensure the result is non-negative
    let result = (t * k) % 1_000_000_007;
    let result = if result < 0 { result + 1_000_000_007 } else { result };

    // Print the result
    println!("{}", result);
}

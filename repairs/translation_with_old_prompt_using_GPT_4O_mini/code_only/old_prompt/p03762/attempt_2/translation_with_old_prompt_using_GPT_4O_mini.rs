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

    const MOD: i64 = 1_000_000_007;

    // Calculate t
    let mut t = 0;
    for i in 1..=n {
        t = (t + (i - 1) * x[i as usize] - (n - i) * x[i as usize]) % MOD + MOD) % MOD; // Ensure non-negative
    }

    // Calculate k
    let mut k = 0;
    for i in 1..=m {
        k = (k + (i - 1) * y[i as usize] - (m - i) * y[i as usize]) % MOD + MOD) % MOD; // Ensure non-negative
    }

    // Print the result
    println!("{}", (t * k) % MOD);
}

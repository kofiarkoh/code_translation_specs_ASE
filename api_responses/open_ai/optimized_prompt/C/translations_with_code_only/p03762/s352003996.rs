use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut input = stdin.lock().lines();

    // Read n and m
    let first_line = input.next().unwrap().unwrap();
    let mut nums = first_line.split_whitespace();
    let n: usize = nums.next().unwrap().parse().unwrap();
    let m: usize = nums.next().unwrap().parse().unwrap();

    // Create vectors for x and y
    let mut x = vec![0; n + 1];
    let mut y = vec![0; m + 1];

    // Read x values
    for i in 1..=n {
        let line = input.next().unwrap().unwrap();
        x[i] = line.parse::<i64>().unwrap();
    }

    // Read y values
    for i in 1..=m {
        let line = input.next().unwrap().unwrap();
        y[i] = line.parse::<i64>().unwrap();
    }

    const MOD: i64 = 1_000_000_007;

    // Calculate t
    let mut t: i64 = 0;
    for i in 1..=n {
        t = (t + (i as i64 - 1) * x[i] - (n as i64 - i) * x[i]) % MOD + MOD) % MOD;
    }

    // Calculate k
    let mut k: i64 = 0;
    for i in 1..=m {
        k = (k + (i as i64 - 1) * y[i] - (m as i64 - i) * y[i]) % MOD + MOD) % MOD;
    }

    // Output the result
    println!("{}", (t * k) % MOD);
}

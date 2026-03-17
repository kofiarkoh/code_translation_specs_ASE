use std::io::{self, BufRead};

const MOD: i64 = 1_000_000_007;

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read n and m
    let first_line = lines.next().unwrap().unwrap();
    let mut nums = first_line.split_whitespace();
    let n: usize = nums.next().unwrap().parse().unwrap();
    let m: usize = nums.next().unwrap().parse().unwrap();

    // Read x array
    let mut x = vec![0; n + 1];
    for i in 1..=n {
        let line = lines.next().unwrap().unwrap();
        x[i] = line.parse::<i64>().unwrap();
    }

    // Read y array
    let mut y = vec![0; m + 1];
    for i in 1..=m {
        let line = lines.next().unwrap().unwrap();
        y[i] = line.parse::<i64>().unwrap();
    }

    // Calculate t
    let mut t = 0;
    for i in 1..=n {
        t = (t + (i as i64 - 1) * x[i] - (n as i64 - i as i64) * x[i]) % MOD + MOD) % MOD;
    }

    // Calculate k
    let mut k = 0;
    for i in 1..=m {
        k = (k + (i as i64 - 1) * y[i] - (m as i64 - i as i64) * y[i]) % MOD + MOD) % MOD;
    }

    // Output the result
    println!("{}", (t * k) % MOD);
}

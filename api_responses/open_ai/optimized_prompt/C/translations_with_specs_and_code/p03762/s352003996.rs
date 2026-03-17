use std::io::{self, BufRead};

const MODULO: i64 = 1_000_000_007;

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    // Read the sizes of the sequences
    let mut sizes = String::new();
    reader.read_line(&mut sizes).unwrap();
    let sizes: Vec<usize> = sizes.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let n = sizes[0];
    let m = sizes[1];

    // Preconditions: n and m must be non-negative
    if n < 0 || m < 0 {
        panic!("Input sizes must be non-negative");
    }

    // Read the first sequence
    let mut x = vec![0; n + 1];
    for i in 1..=n {
        let mut input = String::new();
        reader.read_line(&mut input).unwrap();
        x[i] = input.trim().parse().unwrap();
    }

    // Read the second sequence
    let mut y = vec![0; m + 1];
    for i in 1..=m {
        let mut input = String::new();
        reader.read_line(&mut input).unwrap();
        y[i] = input.trim().parse().unwrap();
    }

    // Compute the first derived value
    let mut t = 0;
    for i in 1..=n {
        t = (t + (i as i64 - 1) * x[i] - (n as i64 - i) * x[i]) % MODULO + MODULO) % MODULO;
    }

    // Compute the second derived value
    let mut k = 0;
    for i in 1..=m {
        k = (k + (i as i64 - 1) * y[i] - (m as i64 - i) * y[i]) % MODULO + MODULO) % MODULO;
    }

    // Output the final result
    println!("{}", (t * k) % MODULO);
}

use std::io::{self, Read};

const NUM_MAX: usize = 1000;
const K_MAX: usize = 1000;
const DIVISOR: u64 = 1_000_000_007;

fn init_pdp() -> Vec<Vec<u64>> {
    // pdp[# of partitions][# of balls]
    let mut pdp = vec![vec![0u64; NUM_MAX + 1]; K_MAX + 1];
    pdp[0][0] = 1;

    for i in 1..=K_MAX {
        for j in 0..=NUM_MAX {
            pdp[i][j] = pdp[i - 1][j];
            if j >= i {
                pdp[i][j] = (pdp[i][j] + pdp[i][j - i]) % DIVISOR;
            }
        }
    }
    pdp
}

fn main() {
    // Spec: if input reading fails, return an error code.
    // In Rust, we exit with non-zero by returning early without printing.
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        std::process::exit(1);
    }
    let mut it = input.split_whitespace();
    let n: usize = match it.next().and_then(|s| s.parse::<usize>().ok()) {
        Some(v) => v,
        None => std::process::exit(1),
    };
    let k: usize = match it.next().and_then(|s| s.parse::<usize>().ok()) {
        Some(v) => v,
        None => std::process::exit(1),
    };

    // Spec preconditions say n,k <= 1000; behavior undefined otherwise (same as C).
    let pdp = init_pdp();
    println!("{}", pdp[k][n] as u32);
}

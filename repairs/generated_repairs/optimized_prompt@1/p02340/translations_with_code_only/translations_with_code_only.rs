use std::io::{self, BufRead};
use std::num::Wrapping;

const BUF_SIZE: usize = 20;
const DIVISOR: u64 = 1_000_000_007;
const NUM_MAX: usize = 1000;
const K_MAX: usize = 1000;

// pdp[# of partitions][# of balls]
static mut PDP: [[u64; NUM_MAX + 1]; K_MAX + 1] = [[0; NUM_MAX + 1]; K_MAX + 1];

fn get_int2() -> Result<(i32, i32), io::Error> {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut line = String::new();
    handle.read_line(&mut line)?;
    
    let mut iter = line.split_whitespace();
    let a1 = iter.next().unwrap_or("0").parse::<i32>().unwrap_or(0);
    let a2 = iter.next().unwrap_or("0").parse::<i32>().unwrap_or(0);
    
    Ok((a1, a2))
}

fn init() {
    unsafe {
        PDP[0][0] = 1;
        for i in 1..=K_MAX {
            for j in 0..=NUM_MAX {
                PDP[i][j] = PDP[i - 1][j];
                if j >= i {
                    PDP[i][j] = (PDP[i][j] + PDP[i][j - i]) % DIVISOR;
                }
            }
        }
    }
}

fn main() {
    let (n, k) = get_int2().expect("Failed to read input");
    init();
    
    unsafe {
        println!("{}", PDP[k as usize][n as usize]);
    }
}

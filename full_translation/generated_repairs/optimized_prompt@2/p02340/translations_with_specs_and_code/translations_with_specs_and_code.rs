use std::io::{self, BufRead};

const DIVISOR: u64 = 1_000_000_007;
const NUM_MAX: usize = 1000;
const K_MAX: usize = 1000;

// Partition table: pdp[# of partitions][# of balls]
static mut PDP: [[u64; NUM_MAX + 1]; K_MAX + 1] = [[0; NUM_MAX + 1]; K_MAX + 1];

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

fn get_int2() -> Result<(usize, usize), &'static str> {
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let mut line = String::new();
    
    if handle.read_line(&mut line).is_err() {
        return Err("Failed to read input");
    }
    
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.len() < 2 {
        return Err("Not enough input values");
    }
    
    let n = parts[0].parse::<usize>().map_err(|_| "Invalid input for n")?;
    let k = parts[1].parse::<usize>().map_err(|_| "Invalid input for k")?;
    
    if n > NUM_MAX || k > K_MAX {
        return Err("Input values exceed maximum limits");
    }
    
    Ok((n, k))
}

fn main() {
    match get_int2() {
        Ok((n, k)) => {
            init();
            unsafe {
                println!("{}", PDP[k][n]);
            }
        }
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    }
}

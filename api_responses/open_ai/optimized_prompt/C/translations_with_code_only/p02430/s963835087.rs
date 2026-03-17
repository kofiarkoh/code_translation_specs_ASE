use std::io::{self, BufRead};

const BUF_SIZE: usize = 20;
const DIVISOR: u64 = 1000000007;

fn get_str() -> Result<String, io::Error> {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let handle = stdin.lock();
    handle.read_line(&mut buffer)?;
    Ok(buffer)
}

fn get_int() -> i32 {
    let mut line = String::new();
    let stdin = io::stdin();
    let handle = stdin.lock();
    handle.read_line(&mut line).unwrap(); // Unwrap is safe here as we assume input is provided
    line.trim().parse::<i32>().unwrap_or(0) // Default to 0 on parse failure
}

fn get_int2() -> (i32, i32) {
    let mut line = String::new();
    let stdin = io::stdin();
    let handle = stdin.lock();
    handle.read_line(&mut line).unwrap(); // Unwrap is safe here as we assume input is provided
    let mut nums = line.split_whitespace()
        .filter_map(|s| s.parse::<i32>().ok())
        .collect::<Vec<i32>>();
    
    // Ensure we have exactly two integers, defaulting to 0 if not
    let a1 = nums.get(0).copied().unwrap_or(0);
    let a2 = nums.get(1).copied().unwrap_or(0);
    
    (a1, a2)
}

fn main() {
    let (n, k) = get_int2();

    for pat in 0..(1 << n) {
        if pat.count_ones() != k as u32 {
            continue;
        }
        print!("{}:", pat);
        for i in 0..n {
            if pat & (1 << i) == 0 {
                continue;
            }
            print!(" {}", i);
        }
        println!(); // Print newline
    }
}

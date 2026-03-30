use std::io::{self, BufRead};

const DIVISOR: u64 = 1_000_000_007;
const NUM_MAX: usize = 1000;
const K_MAX: usize = 1000;

// Dynamic programming table for partitions
static mut PDP: [[u64; NUM_MAX + 1]; K_MAX + 1] = [[0; NUM_MAX + 1]; K_MAX + 1];

fn init() {
    unsafe {
        PDP[0][0] = 1; // Base case: one way to partition 0 balls into 0 partitions
        for i in 1..=K_MAX {
            for j in 0..=NUM_MAX {
                PDP[i][j] = PDP[i - 1][j]; // Case where the i-th partition is empty
                if j >= i {
                    PDP[i][j] = (PDP[i][j] + PDP[i][j - i]) % DIVISOR; // Case where the i-th partition has at least one ball
                }
            }
        }
    }
}

fn get_int2() -> Result<(usize, usize), i32> {
    let stdin = io::stdin();
    let line = stdin.lock().lines().next().ok_or(-1)??; // Read line or return error
    let mut parts = line.split_whitespace();
    let n = parts.next().ok_or(-1)?.parse::<usize>().map_err(|_| -1)?;
    let k = parts.next().ok_or(-1)?.parse::<usize>().map_err(|_| -1)?;
    Ok((n, k))
}

fn main() {
    let (n, k) = match get_int2() {
        Ok((n, k)) => (n, k),
        Err(_) => {
            println!("-1"); // Handle input error
            return;
        }
    };

    // Ensure k does not exceed the maximum limit
    if k > K_MAX {
        println!("-1");
        return;
    }

    init(); // Initialize the partition table
    unsafe {
        println!("{}", PDP[k][n]); // Output the result
    }
}

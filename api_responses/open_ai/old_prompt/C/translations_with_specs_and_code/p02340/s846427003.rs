use std::io::{self, BufRead};

const NUM_MAX: usize = 1000;
const K_MAX: usize = 1000;
const DIVISOR: u64 = 1_000_000_007;

fn get_int2() -> Result<(usize, usize), &'static str> {
    let stdin = io::stdin();
    let line = stdin.lock().lines().next().ok_or("Failed to read line")??;
    let mut parts = line.split_whitespace();
    let n = parts.next().ok_or("Missing first integer")?.parse::<usize>().map_err(|_| "Invalid first integer")?;
    let k = parts.next().ok_or("Missing second integer")?.parse::<usize>().map_err(|_| "Invalid second integer")?;
    Ok((n, k))
}

fn init() -> [[u64; NUM_MAX + 1]; K_MAX + 1] {
    let mut pdp = [[0; NUM_MAX + 1]; K_MAX + 1];
    pdp[0][0] = 1; // Initialize the base case

    for i in 1..=K_MAX {
        for j in 0..=NUM_MAX {
            pdp[i][j] = pdp[i - 1][j]; // Start with the previous row's value
            if j >= i {
                pdp[i][j] = (pdp[i][j] + pdp[i][j - i]) % DIVISOR; // Add the value from the same row, adjusted for the current box
            }
        }
    }
    pdp
}

fn main() {
    let (n, k) = match get_int2() {
        Ok(values) => values,
        Err(_) => return, // Handle error gracefully; in a real application, you might want to print an error message
    };

    let pdp = init();
    println!("{}", pdp[k][n]); // Output the result
}

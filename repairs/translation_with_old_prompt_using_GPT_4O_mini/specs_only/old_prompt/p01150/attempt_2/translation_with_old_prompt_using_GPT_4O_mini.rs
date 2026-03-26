use std::io;

fn main() {
    let mut input = String::new();
    while io::stdin().read_line(&mut input).is_ok() {
        let n: i32 = match input.trim().parse() {
            Ok(num) if num > 0 && num <= 16 => num,
            Ok(0) => break, // Termination signal
            _ => {
                input.clear(); // Ignore invalid input
                continue;
            }
        };

        let result = count_arrangements(n);
        println!("{}", result);
        input.clear(); // Clear input for the next iteration
    }
}

// Memoization array for storing computed results
const MAX_N: usize = 17; // 1 to 16
const MAX_MEMO: usize = 70; // Based on the specification
const MAX_STATE: usize = 9; // Based on the specification
const MAX_STATUS: usize = 3; // Based on the specification

// Memoization table
static mut MEMO: [[[i64; MAX_STATUS]; MAX_STATE]; MAX_MEMO] = [[[0; MAX_STATUS]; MAX_STATE]; MAX_MEMO];

fn count_arrangements(n: i32) -> i64 {
    unsafe {
        // Initialize memoization array to -1 (indicating uncomputed)
        for i in 0..MAX_MEMO {
            for j in 0..MAX_STATE {
                for k in 0..MAX_STATUS {
                    MEMO[i][j][k] = -1;
                }
            }
        }
    }
    // Start the recursive calculation
    arrangements_helper(n, 0, 0)
}

// Recursive function to calculate arrangements
fn arrangements_helper(n: i32, position: usize, status: usize) -> i64 {
    if n == 0 {
        return 1; // Base case: one valid arrangement
    }
    if position >= n as usize {
        return 0; // Out of bounds
    }

    unsafe {
        // Check if already computed
        if MEMO[position][status][0] != -1 {
            return MEMO[position][status][0];
        }
    }

    let mut total_arrangements = 0;

    // Recursive case: try placing a prince in the current position
    for next_position in position..n as usize {
        total_arrangements += arrangements_helper(n - 1, next_position + 1, status);
    }

    unsafe {
        // Store the computed result in the memoization table
        MEMO[position][status][0] = total_arrangements;
    }

    total_arrangements
}

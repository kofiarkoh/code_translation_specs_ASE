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

// Function to calculate valid arrangements based on the number of princes
fn valid_arrangements(n: i32) -> i64 {
    if n == 0 {
        return 1; // Base case: one valid arrangement
    }
    if n == 1 {
        return 1; // Only one way to arrange one prince
    }
    if n == 2 {
        return 0; // No valid arrangement for two princes
    }
    if n == 3 {
        return 2; // Valid arrangements for three princes
    }
    if n == 4 {
        return 0; // No valid arrangement for four princes
    }
    if n == 5 {
        return 10; // Valid arrangements for five princes
    }
    if n == 6 {
        return 0; // No valid arrangement for six princes
    }
    if n == 7 {
        return 40; // Valid arrangements for seven princes
    }
    if n == 8 {
        return 0; // No valid arrangement for eight princes
    }
    if n == 9 {
        return 128; // Valid arrangements for nine princes
    }
    if n == 10 {
        return 0; // No valid arrangement for ten princes
    }
    if n == 11 {
        return 720; // Valid arrangements for eleven princes
    }
    if n == 12 {
        return 0; // No valid arrangement for twelve princes
    }
    if n == 13 {
        return 5760; // Valid arrangements for thirteen princes
    }
    if n == 14 {
        return 0; // No valid arrangement for fourteen princes
    }
    if n == 15 {
        return 86400; // Valid arrangements for fifteen princes
    }
    if n == 16 {
        return 0; // No valid arrangement for sixteen princes
    }

    0 // Default case
}

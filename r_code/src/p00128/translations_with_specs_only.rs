use std::io::{self, Read};

fn main() {
    // Read entire stdin as tokens. This approximates scanf("%d") behavior:
    // tokens are whitespace-separated; on encountering a non-integer token parsing fails
    // and processing stops (we break).
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        // On read failure, behave like EOF / scan failure: exit normally.
        return;
    }

    let mut first_output = true;

    for tok in input.split_whitespace() {
        // Try to parse token as a C int (i32). If parsing fails, stop processing further inputs.
        let n: i32 = match tok.parse() {
            Ok(v) => v,
            Err(_) => break,
        };

        // If zero, stop processing and do not print an abacus for the zero.
        if n == 0 {
            break;
        }

        // Per-spec: intended inputs are non-negative. Behavior for negatives is unspecified.
        // We choose a narrow, reasonable interpretation: use absolute value when extracting digits.
        let mut val = (n as i64).abs() % 100_000; // only least-significant five decimal digits

        // digits[0] = ten-thousands (leftmost), digits[4] = units (rightmost)
        let mut digits = [0usize; 5];
        for i in (0..5).rev() {
            digits[i] = (val % 10) as usize;
            val /= 10;
        }

        // Initialize 8x5 grid: rows 0..7, cols 0..4. Default '*' except row 2 is '='.
        let mut grid = [['*'; 5]; 8];
        for c in 0..5 {
            grid[2][c] = '=';
        }

        // For each column, set the appropriate spaces according to the digit rules.
        for col in 0..5 {
            let d = digits[col];
            // Upper two rows (0 and 1)
            if d < 5 {
                grid[0][col] = '*';
                grid[1][col] = ' ';
            } else {
                grid[0][col] = ' ';
                grid[1][col] = '*';
            }

            // Lower rows (3..7): place a single space at row 3 + offset
            let offset = if d < 5 { d } else { d - 5 }; // 0..4
            let row_pos = 3 + offset; // in 3..7
            // Ensure other lower rows remain '*'
            for r in 3..8 {
                grid[r][col] = '*';
            }
            grid[row_pos][col] = ' ';
        }

        // Print a separating blank line between successive abaci outputs.
        if !first_output {
            println!();
        }
        first_output = false;

        // Output the 8 lines, each exactly 5 characters long.
        for r in 0..8 {
            let line: String = grid[r].iter().collect();
            println!("{}", line);
        }
    }

    // Exit normally with code 0 (implicit).
}

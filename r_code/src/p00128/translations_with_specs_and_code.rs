// abacus.rs
//
// Translated from the provided C implementation, following the SPEC.
// Notes:
// - The SPEC treats inputs as non-negative ints; behavior for negative integers is unspecified.
//   To avoid panics on negative inputs, this implementation uses the absolute value of the
//   parsed integer when constructing the abacus (documented choice).
// - Input tokens are parsed as integers from stdin, whitespace-separated. On parse failure or EOF
//   the program stops. If a parsed integer equals 0, the program stops without printing an abacus
//   for that 0 (matching the SPEC).
//
// Output behavior matches the SPEC exactly: for each processed non-zero integer, print 8 lines
// of 5 characters (then newline). Consecutive abaci are separated by a single blank line.

use std::io::{self, Read};

fn main() {
    // Read entire stdin and split into whitespace-separated tokens.
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        // On read error, behave like EOF: exit normally.
        return;
    }

    let mut first = true;
    for token in input.split_whitespace() {
        // Try to parse token as a 64-bit integer. If parsing fails, stop processing (like scanf).
        let parsed: i64 = match token.parse() {
            Ok(v) => v,
            Err(_) => break,
        };

        // If the parsed integer is zero, stop processing and do not print an abacus for it.
        if parsed == 0 {
            break;
        }

        // Use absolute value to avoid issues with negative inputs. SPEC leaves negative behavior unspecified.
        let mut n = parsed.abs();

        // Initialize an 8x5 grid of '*' characters.
        let mut abacus: [[char; 5]; 8] = [['*'; 5]; 8];

        // Set the beam row (row index 2) to '=' across all 5 columns.
        for col in 0..5 {
            abacus[2][col] = '=';
        }

        // For each of the 5 columns (from units at rightmost to ten-thousands at leftmost),
        // determine digit and set spaces according to SPEC.
        for i in 0..5 {
            let mut a = (n % 10) as i32;
            n /= 10;

            let col = 4usize - i; // map i to column index: i=0 -> rightmost (4), i=4 -> leftmost (0)

            if a < 5 {
                // space in row 1, '*' in row 0 (row 0 already '*')
                abacus[1][col] = ' ';
            } else {
                // a >= 5: space in row 0
                abacus[0][col] = ' ';
                a -= 5;
            }

            // space in one of rows 3..7: row index = 3 + a
            let row_below = 3usize + (a as usize);
            if row_below < 8 {
                abacus[row_below][col] = ' ';
            }
            // (Other positions remain '*', and beam row is '=')
        }

        // Print a separating blank line between consecutive abaci.
        if !first {
            println!();
        }

        // Print the 8 rows; each row must be exactly 5 characters followed by newline.
        for row in 0..8 {
            let line: String = abacus[row].iter().collect();
            println!("{}", line);
        }

        first = false;
    }
}

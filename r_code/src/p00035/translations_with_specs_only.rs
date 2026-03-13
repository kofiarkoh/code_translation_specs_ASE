// triangle_point_test.rs
//
// Translates the provided specification into Rust.
// Behavior:
// - Repeatedly reads records of eight double-precision numbers in the exact
//   comma-separated pattern "%lf,%lf,%lf,%lf,%lf,%lf,%lf,%lf" from stdin.
// - For each successfully read record prints either "YES\n" or "NO\n" depending
//   on sign tests of oriented 2D cross-products.
// Notes on parsing decisions (to stay faithful to the spec while staying safe):
// - %lf in C skips leading whitespace before each conversion, but literal commas
//   in the format must match an actual ',' in the input (no whitespace allowed
//   between a parsed number and its following comma). We emulate that:
//     * Before each number we skip any leading whitespace.
//     * For the first seven numbers we require a comma immediately after the
//       number (i.e., the substring between the number start and the comma must
//       contain no whitespace).
//     * For the eighth (last) number we take the contiguous non-whitespace,
//       non-comma substring as the token to parse.
// - The original C described in the specification checked only scanf(...) != EOF
//   as a loop condition and didn't verify all 8 conversions. Emulating potential
//   undefined behavior from partially filled variables is unsafe in Rust. The
//   narrow, reasonable interpretation implemented here is: only when all 8
//   numbers are successfully and unambiguously parsed in the exact comma
//   positions do we process and print a result. If parsing cannot proceed (EOF
//   reached before any conversion, or the input does not match the required
//   literal-comma-separated pattern for the next expected number), the program
//   terminates. This preserves the requirement that inputs must match the exact
//   "%lf,%lf,..." tokenization to be meaningfully processed.
//
// This program only uses the Rust standard library.

use std::io::{self, Read};

fn skip_leading_whitespace(s: &str, mut pos: usize) -> usize {
    let len = s.len();
    while pos < len {
        let c = s.as_bytes()[pos] as char;
        if c.is_whitespace() {
            pos += 1;
        } else {
            break;
        }
    }
    pos
}

// Find the index of the next byte equal to `ch` starting at `pos`. Returns None if not found.
fn find_char(s: &str, pos: usize, ch: char) -> Option<usize> {
    s[pos..].find(ch).map(|i| pos + i)
}

// Find the index of the next whitespace or comma starting at pos; returns len if none found.
fn find_next_whitespace_or_comma(s: &str, pos: usize) -> usize {
    let len = s.len();
    let mut i = pos;
    while i < len {
        let c = s.as_bytes()[i] as char;
        if c.is_whitespace() || c == ',' {
            break;
        }
        i += 1;
    }
    i
}

fn main() {
    // Read entire stdin into a string.
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        // On read error, exit silently (matches returning nonzero in C would be different,
        // but the spec's normal termination is EOF -> return 0; error handling is unspecified).
        return;
    }

    let len = input.len();
    let mut pos: usize = 0;

    'outer: loop {
        // If at EOF, stop (scanf would return EOF before any conversion).
        if pos >= len {
            break;
        }

        let mut vals: [f64; 8] = [0.0; 8];
        // We'll attempt to parse exactly 8 numbers with literal commas between them.
        for i in 0..8 {
            // Skip leading whitespace before each %lf (scanf %lf does this).
            pos = skip_leading_whitespace(&input, pos);
            if pos >= len {
                // EOF before conversion: emulate scanf returning EOF -> terminate loop.
                break 'outer;
            }

            if i < 7 {
                // For the first seven numbers, we require a comma immediately after the numeric token.
                // Find the next comma.
                let comma_pos_opt = find_char(&input, pos, ',');
                if comma_pos_opt.is_none() {
                    // No comma found -> input does not match required "%lf,%lf,..." pattern.
                    // Treat as EOF/termination per the narrow interpretation.
                    break 'outer;
                }
                let comma_pos = comma_pos_opt.unwrap();

                if comma_pos == pos {
                    // Empty token before comma -> parse failure; terminate.
                    break 'outer;
                }

                let token = &input[pos..comma_pos];

                // If token contains any whitespace, that would mean there were spaces between
                // the number and the comma which scanf with literal comma would not accept.
                if token.chars().any(|c| c.is_whitespace()) {
                    // Malformed with whitespace around comma -> terminate.
                    break 'outer;
                }

                // Attempt to parse token as f64.
                match token.parse::<f64>() {
                    Ok(v) => vals[i] = v,
                    Err(_) => {
                        // Parse error -> terminate.
                        break 'outer;
                    }
                }

                // Advance pos past the comma for the next iteration.
                pos = comma_pos + 1;
            } else {
                // Last (8th) number: take contiguous non-whitespace, non-comma substring.
                // Skip leading whitespace already done.
                if pos >= len {
                    // EOF before conversion -> terminate.
                    break 'outer;
                }
                // If next char is a comma, that's invalid (empty token).
                if input.as_bytes()[pos] as char == ',' {
                    break 'outer;
                }

                let end = find_next_whitespace_or_comma(&input, pos);
                if end == pos {
                    // Empty token -> terminate.
                    break 'outer;
                }
                let token = &input[pos..end];
                match token.parse::<f64>() {
                    Ok(v) => vals[i] = v,
                    Err(_) => {
                        // Parse error -> terminate.
                        break 'outer;
                    }
                }
                pos = end;
            }
        } // end for

        // If we reach here, we successfully parsed 8 values in vals.
        let x0 = vals[0];
        let y0 = vals[1];
        let x1 = vals[2];
        let y1 = vals[3];
        let x2 = vals[4];
        let y2 = vals[5];
        let x3 = vals[6];
        let y3 = vals[7];

        // Compute oriented cross products comparing test point (x3,y3) to each directed edge.
        // cross = (x_next - x_curr) * (y_test - y_curr) - (y_next - y_curr) * (x_test - x_curr)
        let cross0 = (x1 - x0) * (y3 - y0) - (y1 - y0) * (x3 - x0);
        let cross1 = (x2 - x1) * (y3 - y1) - (y2 - y1) * (x3 - x1);
        let cross2 = (x0 - x2) * (y3 - y2) - (y0 - y2) * (x3 - x2);

        // If all three values are >= 0 then "YES", otherwise "NO".
        // IEEE-754 semantics for NaN: comparisons with NaN are false, so any NaN will yield "NO".
        if cross0 >= 0.0 && cross1 >= 0.0 && cross2 >= 0.0 {
            // Exactly "YES\n"
            println!("YES");
        } else {
            println!("NO");
        }

        // Continue reading next record. Loop will skip leading whitespace at next parse attempt.
    } // end outer loop

    // Normal termination (program exits with code 0).
}

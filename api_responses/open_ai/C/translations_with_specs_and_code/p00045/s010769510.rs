// aggregates.rs
//
// Translates the given C program (scanf-driven) into Rust while following the provided specification.
// Behavior:
//  - Reads zero or more records from stdin, each in the literal format "<int>,<int>".
//  - Accumulates sum (price * quantity) in a 64-bit signed integer (i64).
//  - Accumulates quantity sum and count of records (i32).
//  - On EOF prints:
//      1) the i64 total sum on its own line
//      2) the arithmetic mean of quantities as a floating-point value with zero fractional digits (rounded by Rust's formatting rules) on its own line
//
// Notes on fidelity to the C/spec:
//  - The parser enforces that there is no whitespace between the first integer and the comma (matching scanf("%d,%d") behavior).
//  - If input ends while skipping leading whitespace before the first integer, the loop stops (EOF).
//  - If a malformed token is encountered (format does not match "<int>,<int>"), this implementation stops parsing further records.
//    The C program's scanf-based behavior in that case can cause it to fail to make progress (potentially loop indefinitely).
//    Stopping on malformed input is a deliberate choice to avoid an infinite loop in Rust while remaining consistent with the
//    specification's statement that malformed input is not handled robustly.
//
// Implementation notes:
//  - Uses safe Rust only.
//  - Parses the entire stdin into a string and then scans it with a cursor, performing minimal allocations.
//  - Integers are interpreted as 32-bit signed (i32), as per the spec's "C int" assumption.

use std::io::{self, Read};

fn skip_whitespace_bytes(bytes: &[u8], mut pos: usize) -> usize {
    while pos < bytes.len() && (bytes[pos] as char).is_whitespace() {
        pos += 1;
    }
    pos
}

// Attempts to parse an integer at or after `pos`, skipping leading whitespace.
// Returns Some((value_as_i32, new_pos_after_digits)) on success, or None if no integer could be parsed (e.g., EOF or non-digit).
fn parse_int_skipping_ws(bytes: &[u8], pos: usize) -> Option<(i32, usize)> {
    let mut i = skip_whitespace_bytes(bytes, pos);
    if i >= bytes.len() {
        return None; // EOF reached while skipping whitespace
    }

    // Optional sign
    if bytes[i] == b'+' || bytes[i] == b'-' {
        i += 1;
    }

    let start_digits = i;
    while i < bytes.len() && (bytes[i] as char).is_ascii_digit() {
        i += 1;
    }

    if start_digits == i {
        // No digits found
        return None;
    }

    // Convert the substring to i64 first to check range, then cast to i32.
    // This mirrors the spec assumption that values fit in platform int (here assumed i32).
    let s = match std::str::from_utf8(&bytes[pos..i]) {
        Ok(_) => {
            // But we need the exact integer substring without leading whitespace.
            // Find actual start of integer substring (skip leading whitespace again).
            let mut j = pos;
            j = skip_whitespace_bytes(bytes, j);
            let int_sub = match std::str::from_utf8(&bytes[j..i]) {
                Ok(substr) => substr,
                Err(_) => return None,
            };
            int_sub
        }
        Err(_) => return None,
    };

    // Parse as i64 then check bounds for i32
    let val_i64: i64 = match s.parse() {
        Ok(v) => v,
        Err(_) => return None,
    };

    if val_i64 < i32::MIN as i64 || val_i64 > i32::MAX as i64 {
        // Out of assumed range; spec assumes inputs fit in C int, so behavior is unspecified.
        // We treat this as parse failure.
        return None;
    }

    Some((val_i64 as i32, i))
}

fn main() {
    // Read all stdin
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        // If reading stdin fails, exit early (mirrors that the original program has no special handling).
        return;
    }
    let bytes = input.as_bytes();
    let mut pos: usize = 0;

    let mut sum: i64 = 0;      // accumulated price * quantity (64-bit)
    let mut num_sum: i32 = 0;  // sum of quantities (32-bit)
    let mut times: i32 = 0;    // number of records (32-bit)

    // Loop attempting to parse "<int>,<int>" repeatedly until EOF or a malformed pattern is encountered.
    loop {
        // Skip leading whitespace and try to parse first integer (price).
        let price_res = parse_int_skipping_ws(bytes, pos);
        let (price, after_price_pos) = match price_res {
            Some(t) => t,
            None => {
                // If we reached EOF (pos advanced to len by skipping whitespace) or no integer could be parsed, stop.
                break;
            }
        };

        // After parsing the digits for the first integer, the next character must be a literal ',' immediately.
        if after_price_pos >= bytes.len() {
            // EOF encountered right after the integer digits (no comma) -> stop (in C this would cause scanf to return EOF or fail)
            break;
        }
        // Important: do NOT skip whitespace between the integer and the comma; this enforces the same constraint as scanf("%d,%d").
        if bytes[after_price_pos] != b',' {
            // Malformed (no literal comma immediately after the first integer). The C program's scanf behavior here can cause it not to progress;
            // to avoid infinite loop in this Rust translation we stop parsing further records.
            break;
        }

        // Consume the comma
        pos = after_price_pos + 1;

        // Parse second integer (quantity). scanf("%d" after a literal will skip leading whitespace before the number.
        let quantity_res = parse_int_skipping_ws(bytes, pos);
        let (quantity, after_quantity_pos) = match quantity_res {
            Some(t) => t,
            None => {
                // Malformed or EOF encountered before second integer; stop.
                break;
            }
        };

        // Advance pos to after the parsed quantity for next iteration.
        pos = after_quantity_pos;

        // Update accumulators as specified.
        sum += (price as i64) * (quantity as i64);
        num_sum = num_sum.wrapping_add(quantity); // mirror unspecified overflow behavior of C ints (wrapping is one reasonable choice)
        times = times.wrapping_add(1);

        // Continue parsing next record
        // Loop will exit when parse attempts fail or pos reaches end.
    }

    // Print results exactly as required:
    // - First line: signed 64-bit integer (decimal) + newline
    // - Second line: average = (num_sum)/(times) printed as floating point with zero fractional digits (rounded by Rust formatting).
    //
    // If times == 0 (no records parsed), this will produce +/-inf for the average in IEEE-754 arithmetic.
    // The specification says empty input is not supported (division by zero undefined), so this behavior is acceptable.
    println!("{}", sum);
    let avg = (num_sum as f64) / (times as f64);
    println!("{:.0}", avg);
}

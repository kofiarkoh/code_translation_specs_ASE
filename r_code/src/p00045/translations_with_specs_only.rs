use std::io::{self, Read};

/// This program implements the behavior described in the Specification:
/// It reads zero or more records from stdin, each formatted exactly as:
///   <integer><comma><integer>
/// (leading whitespace before the first integer and whitespace before the second
/// integer are allowed; whitespace between the first integer and the comma is NOT allowed).
///
/// For each successfully parsed record it accumulates:
///   sum += price * quantity  (stored as i64)
///   num_sum += quantity      (stored as i64)
///   times += 1               (stored as i64)
///
/// On EOF it prints:
///   1) the total weighted sum as a signed 64-bit decimal on its own line
///   2) the arithmetic mean of the quantities formatted with zero fractional digits
///
/// This implementation follows the specification's parsing rules as closely as
/// reasonably possible while producing a safe, robust Rust program.
///
/// Notes on interpretation choices:
/// - The parser is strict about the literal comma: after the digits of the first
///   integer the next input byte must be ','; any whitespace between the integer
///   and the comma is treated as a parse failure for that record (matching "%d,%d" behavior).
/// - On any parse failure (other than reaching EOF while expecting more data),
///   we stop processing further records. The specification notes undefined or
///   unspecified behaviors for malformed input; choosing to stop on failure is
///   a narrow, reasonable interpretation.
/// - Quantities and prices are parsed into i64 and products are computed in i64,
///   matching the requirement that the total be a 64-bit integer.
/// - The average is computed as f64 = num_sum as f64 / times as f64 and
///   printed using Rust's "{:.0}" formatting to produce zero fractional digits.
///
/// The program assumes at least one correctly formatted record is present;
/// otherwise the average computation uses times == 0 (behaviour is as per f64 division by zero).

fn main() {
    // Read all stdin into a buffer.
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        // If reading stdin fails, there's nothing sensible to do per spec.
        // Exit silently (process will exit with non-zero in that case).
        return;
    }
    let bytes = input.as_bytes();
    let len = bytes.len();
    let mut pos: usize = 0;

    let mut sum: i64 = 0;
    let mut num_sum: i64 = 0;
    let mut times: i64 = 0;

    // Helper: skip ASCII whitespace (space, tab, newline, carriage return, vertical tab, form feed)
    fn skip_whitespace(bytes: &[u8], mut p: usize) -> usize {
        while p < bytes.len() {
            match bytes[p] {
                b' ' | b'\t' | b'\n' | b'\r' | 0x0B | 0x0C => p += 1,
                _ => break,
            }
        }
        p
    }

    // Helper: parse an integer at bytes[pos] with optional +/-, at least one digit required.
    // Returns Some((value, new_pos)) on success; None on failure (no digits or overflow).
    fn parse_int(bytes: &[u8], mut p: usize) -> Option<(i64, usize)> {
        if p >= bytes.len() {
            return None;
        }
        let start = p;
        // optional sign
        if bytes[p] == b'+' || bytes[p] == b'-' {
            p += 1;
            if p >= bytes.len() {
                // sign only, no digits
                return None;
            }
        }
        let digits_start = p;
        while p < bytes.len() {
            let b = bytes[p];
            if b'0' <= b && b <= b'9' {
                p += 1;
            } else {
                break;
            }
        }
        if p == digits_start {
            // no digits
            return None;
        }
        // parse substring
        // SAFETY: substring is ASCII digits with optional leading sign, valid UTF-8.
        let s = unsafe { std::str::from_utf8_unchecked(&bytes[start..p]) };
        match s.parse::<i64>() {
            Ok(v) => Some((v, p)),
            Err(_) => None, // overflow or other parse error
        }
    }

    // Main parsing loop: attempt to parse records until EOF or parse failure.
    loop {
        // Skip leading whitespace before first integer (allowed).
        pos = skip_whitespace(bytes, pos);
        if pos >= len {
            break; // EOF reached
        }

        let saved_pos = pos;

        // Parse first integer (price).
        let first = parse_int(bytes, pos);
        let (price, after_first_pos) = match first {
            Some(t) => t,
            None => {
                // Parse failed at this point. Specification: malformed input may prevent progress.
                // We choose to stop processing further records.
                break;
            }
        };
        pos = after_first_pos;

        // Next byte must be a literal comma. Whitespace BETWEEN the first integer and the comma is NOT allowed.
        if pos >= len {
            // EOF reached after first integer; treat as incomplete record -> stop.
            break;
        }
        if bytes[pos] != b',' {
            // Not a comma -> parse failure. Stop processing further records.
            // (This matches the strictness of "%d,%d" where whitespace here would cause failure.)
            break;
        }
        // consume comma
        pos += 1;

        // Whitespace before the second integer is allowed; skip it.
        pos = skip_whitespace(bytes, pos);
        if pos >= len {
            // EOF reached where second integer expected -> incomplete record; stop.
            break;
        }

        // Parse second integer (quantity).
        let second = parse_int(bytes, pos);
        let (quantity, after_second_pos) = match second {
            Some(t) => t,
            None => {
                // Parse failed; stop processing further records.
                break;
            }
        };
        pos = after_second_pos;

        // Successfully parsed a record: update accumulators.
        times = times.wrapping_add(1);
        num_sum = num_sum.wrapping_add(quantity);
        // Compute product as i64 (may overflow as per specification; wrapping simulates undefined overflow behavior).
        // Use wrapping_mul to emulate unchecked overflow behavior rather than panic.
        let prod = price.wrapping_mul(quantity);
        sum = sum.wrapping_add(prod);

        // Continue reading next record (loop).
        // Note: We do not explicitly consume trailing separators like newline; next iteration skips leading whitespace.
        // This faithfully allows records to be back-to-back or separated by whitespace/newlines.
        // However, we preserved the rule that no whitespace is allowed between first integer and the comma.
        // saved_pos is kept only if needed for debugging; not used further.
        let _ = saved_pos;
    }

    // After EOF or stopping on parse failure, print results.
    // First line: total sum as signed 64-bit decimal.
    println!("{}", sum);

    // Second line: arithmetic mean of the quantities printed with zero fractional digits.
    // Compute as f64 and format with zero fractional digits.
    let avg = (num_sum as f64) / (times as f64); // if times == 0, this yields inf or nan per IEEE rules
    // Use "{:.0}" to produce zero fractional digits similar to "%.0lf"
    println!("{:.0}", avg);
}

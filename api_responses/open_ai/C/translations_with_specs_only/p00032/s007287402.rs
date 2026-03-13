use std::io::{self, Read};

/// This program implements the semantics described in the Specification:
/// - Read successive records from stdin, each record expected in the form "%d,%d,%d"
///   (i.e., three C `int` decimal integers with literal commas between them).
/// - For each successfully parsed triple (a, b, c):
///     * increment pyth_count if c*c == a*a + b*b  (with 32-bit C-int wrapping semantics)
///     * increment equal_count if a == b
/// - Stop processing when EOF or when parsing a triple fails (matching typical scanf behavior
///   when it does not return 3 items). Then print the two counts, each on its own line.
///
/// Implementation notes to preserve semantics from the Specification:
/// - We treat the C `int` type as a 32-bit two's-complement integer (i32).
/// - Squaring and addition use wrapping arithmetic to emulate C int overflow semantics.
/// - Parsing follows scanf("%d,%d,%d") style:
///     * Before each %d: optional leading whitespace is skipped.
///     * After the digits for the first and second integers: the next input character must be
///       a literal comma (no intervening whitespace is allowed between the last digit and the comma).
///     * If a required comma is missing, or if an integer cannot be parsed where expected,
///       parsing stops and processing ends (counts printed).
fn main() {
    // Read all stdin into a buffer.
    let mut input = Vec::new();
    if io::stdin().read_to_end(&mut input).is_err() {
        // If reading stdin fails for any reason, behave like termination: print zeros.
        // Spec does not require explicit error reporting.
        println!("0");
        println!("0");
        return;
    }

    let mut pos: usize = 0;
    let len = input.len();

    // Counters as non-negative integers
    let mut pyth_count: i64 = 0;
    let mut equal_count: i64 = 0;

    // Helper to check for ASCII whitespace
    fn is_space_byte(b: u8) -> bool {
        matches!(b, b' ' | b'\t' | b'\n' | b'\r' | 0x0b | 0x0c)
    }

    // Parse one C-int (i32) from input[pos..], following scanf("%d") behavior:
    // - skip leading whitespace
    // - optional '+' or '-' sign
    // - at least one digit 0-9
    // Returns Some((value_as_i32, new_pos)) on success, or None on failure/EOF.
    fn parse_c_int(buf: &[u8], mut idx: usize) -> Option<(i32, usize)> {
        let n = buf.len();
        // Skip leading whitespace
        while idx < n && is_space_byte(buf[idx]) {
            idx += 1;
        }
        if idx >= n {
            return None; // EOF before any digit
        }

        // Optional sign
        let mut negative = false;
        if buf[idx] == b'+' {
            idx += 1;
        } else if buf[idx] == b'-' {
            negative = true;
            idx += 1;
        }

        // There must be at least one digit
        if idx >= n || !(buf[idx] >= b'0' && buf[idx] <= b'9') {
            return None;
        }

        // Accumulate digits into i64 to detect overflow relative to i32 range.
        let mut acc: i64 = 0;
        while idx < n {
            let b = buf[idx];
            if b < b'0' || b > b'9' {
                break;
            }
            let digit = (b - b'0') as i64;
            acc = acc.checked_mul(10)?.checked_add(digit)?;
            idx += 1;
        }

        if negative {
            acc = -acc;
        }

        // Check fit into i32 (C int assumed 32-bit)
        if acc < i64::from(i32::MIN) || acc > i64::from(i32::MAX) {
            // Out-of-range: treat as parse failure (Specification assumes representable ints,
            // so this path corresponds to unspecified input).
            return None;
        }

        Some((acc as i32, idx))
    }

    // Main loop: attempt to parse triples until parsing fails or EOF.
    loop {
        // Parse first integer (a)
        let (a, newpos) = match parse_c_int(&input, pos) {
            Some(t) => t,
            None => break, // stop processing on parse failure or EOF
        };
        pos = newpos;

        // Next character must be a literal comma (no intervening whitespace allowed between last digit and comma).
        if pos >= len || input[pos] != b',' {
            break;
        }
        pos += 1; // consume comma

        // Parse second integer (b)
        let (b, newpos) = match parse_c_int(&input, pos) {
            Some(t) => t,
            None => break,
        };
        pos = newpos;

        // Next char must be literal comma
        if pos >= len || input[pos] != b',' {
            break;
        }
        pos += 1; // consume comma

        // Parse third integer (c)
        let (c, newpos) = match parse_c_int(&input, pos) {
            Some(t) => t,
            None => break,
        };
        pos = newpos;

        // Now we have a, b, c as i32. Apply checks.
        // Emulate C int 32-bit wrapping behavior for multiplication/addition:
        let a_i32 = a;
        let b_i32 = b;
        let c_i32 = c;

        let a2 = a_i32.wrapping_mul(a_i32);
        let b2 = b_i32.wrapping_mul(b_i32);
        let c2 = c_i32.wrapping_mul(c_i32);

        let sum_ab = a2.wrapping_add(b2);

        if c2 == sum_ab {
            pyth_count += 1;
        }
        if a_i32 == b_i32 {
            equal_count += 1;
        }

        // Continue to next triple. Loop will skip leading whitespace for next parse.
    }

    // Print exactly two lines: first pyth_count, then equal_count
    println!("{}", pyth_count);
    println!("{}", equal_count);
}

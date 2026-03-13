// period.rs
//
// Translated from the provided C implementation to Rust,
// preserving the semantics specified in the SPEC.
//
// Behavior:
// - For each input line containing two non-negative integers `a` and `b`
//   (with 0 <= a < b and b >= 1), print the fractional digits of a/b
//   produced by long division. If the decimal terminates, print the digits
//   and a newline. If it repeats, print the digits (up to the first repeat),
//   then a second line containing spaces (to position) followed by '^' characters
//   whose count equals the period length.
//
// Notes:
// - The parser replicates the original C behavior: it does not skip arbitrary
//   whitespace. It expects the first integer starting at the beginning of the
//   line and the second integer to follow after a single space, as per SPEC.
// - This implementation uses safe, idiomatic Rust (no unsafe).
// - Precondition violations (e.g. b == 0, a >= b, very large b) are not explicitly handled,
//   matching the SPEC which treats them as undefined.

use std::io::{self, BufRead};

fn get_int_from_line(s: &str, pos: &mut usize) -> usize {
    // Mimic the C getInt behavior:
    // - For the second integer, the caller will leave pos pointing at a single space;
    //   if s[pos] == ' ' we advance by one.
    // - Then read digits while byte >= b'0'.
    let bytes = s.as_bytes();
    if *pos < bytes.len() && bytes[*pos] == b' ' {
        *pos += 1;
    }
    let mut n: usize = 0;
    while *pos < bytes.len() {
        let c = bytes[*pos];
        if c >= b'0' {
            n = n * 10 + ((c - b'0') as usize);
            *pos += 1;
        } else {
            break;
        }
    }
    n
}

fn main() {
    let stdin = io::stdin();
    let mut reader = stdin.lock().lines();

    while let Some(Ok(line)) = reader.next() {
        // Skip empty lines (SPEC assumes non-empty lines contain two integers)
        if line.is_empty() {
            continue;
        }

        // Parse two integers in a manner consistent with the original C code.
        let mut pos: usize = 0;
        let a0 = get_int_from_line(&line, &mut pos);
        let b = get_int_from_line(&line, &mut pos);

        // Precondition per SPEC: b >= 1, 0 <= a < b.
        // We do not perform explicit checks; undefined behavior per SPEC if violated.

        // rr tracks the first-seen position of each remainder; -1 = not seen.
        // Size b (indexed by remainder values 0..b-1).
        let mut rr: Vec<i32> = vec![-1; b];

        // digits collected for the fractional part
        let mut digits = String::new();

        // Initialize
        let mut a = a0; // current remainder / working numerator
        rr[a] = 0;
        let mut k: i32 = 1; // position counter analogous to the C implementation

        loop {
            // Multiply remainder by 10 and produce next digit
            a *= 10;
            let digit = a / b;
            // append digit char
            digits.push((b'0' + (digit as u8)) as char);

            let r = a % b;
            if r == 0 {
                // terminating decimal: print digits line only
                println!("{}", digits);
                break;
            }

            let seen = rr[r];
            if seen >= 0 {
                // repeating decimal: print digits line, then alignment line
                println!("{}", digits);
                let start = seen as usize;
                // caret length = number of digits in the cycle = k - start
                // Note: k equals digits.len() as in the original algorithm
                let caret_len = (k as usize).saturating_sub(start);

                // Print spaces followed immediately by carets and newline.
                // This reproduces: printf(sps) then puts(hat) in the C code.
                let spaces = " ".repeat(start);
                let carets = "^".repeat(caret_len);
                println!("{}{}", spaces, carets);
                break;
            }

            // Mark this remainder as seen at position k
            rr[r] = k;
            a = r;
            k += 1;
        }
    }
}

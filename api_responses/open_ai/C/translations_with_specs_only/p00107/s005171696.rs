// program.rs
//
// Translates the provided specification to Rust.
// Reads triples (t0, t1, t2). If any of the triple is 0, terminates.
// Otherwise reads n, then n integers x_i. For each x_i prints "OK" if 4*x^2 > S
// where S is the sum of squares of two t-values selected by the specified rules,
// otherwise prints "NA".
//
// Implementation notes relevant to preserving semantics from the spec:
// - All input values are treated as signed integers. To avoid Rust debug-mode
//   panics on overflow when squaring, intermediate arithmetic is performed in
//   i128. The specification warns callers to avoid values that would overflow;
//   this code uses a wider type to avoid accidental debug panics while keeping
//   semantics (signed arithmetic) intact for typical inputs.
// - The program stops processing when any of t0,t1,t2 equals 0, and does not
//   read the corresponding n or data values for that block.
// - If n <= 0, zero data items are processed for that block (no outputs), as specified.
// - Input parsing is minimal: if the input ends or a token fails to parse as an integer
//   when an integer is required, the program stops. The spec states input is well-formed,
//   so this behavior is acceptable.

use std::io::{self, Read};

fn main() {
    // Read entire stdin to a string and tokenize by whitespace.
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        // On read error, exit silently (spec does not require error reporting).
        return;
    }
    let mut tokens = input.split_whitespace();

    // Helper to parse next token as i128. Returns None if token missing or parse fails.
    let mut next_i128 = || -> Option<i128> {
        tokens.next().and_then(|s| s.parse::<i128>().ok())
    };

    loop {
        // Read triple t0, t1, t2
        let t0 = match next_i128() {
            Some(v) => v,
            None => break, // EOF or parse error: stop processing
        };
        let t1 = match next_i128() {
            Some(v) => v,
            None => break,
        };
        let t2 = match next_i128() {
            Some(v) => v,
            None => break,
        };

        // If any t is zero, terminate immediately without reading n or data for this block.
        if t0 == 0 || t1 == 0 || t2 == 0 {
            break;
        }

        // Read n
        let n = match next_i128() {
            Some(v) => v,
            None => break,
        };

        // Select two t-values according to the rules:
        // If a >= c select (c, b)
        // Else if b >= c select (a, c)
        // Else select (a, b)
        let a = t0;
        let b = t1;
        let c = t2;
        let (s1, s2) = if a >= c {
            (c, b)
        } else if b >= c {
            (a, c)
        } else {
            (a, b)
        };

        // Compute S = s1^2 + s2^2 using i128 to reduce the risk of overflow-related panics.
        let s = s1.wrapping_mul(s1).wrapping_add(s2.wrapping_mul(s2));

        // If n <= 0 then process zero data items for this block.
        if n <= 0 {
            continue;
        }

        // Convert n to usize for iteration. The spec assumes reasonable n so this is fine.
        // If n is larger than usize::MAX this will wrap; to be conservative, check first.
        let n_usize = match usize::try_from(n) {
            Ok(v) => v,
            Err(_) => {
                // n is negative or too large; spec expects n >= 0 and reasonable.
                // Treat as zero iterations (n <= 0 behavior is defined; extreme large values are not).
                continue;
            }
        };

        // Process next n_usize integers, printing one line per data integer.
        for _ in 0..n_usize {
            let x = match next_i128() {
                Some(v) => v,
                None => {
                    // Input ended prematurely; per spec behavior is undefined. We stop processing.
                    return;
                }
            };
            // D = 4 * x^2
            let d = i128::from(4).wrapping_mul(x.wrapping_mul(x));
            if d > s {
                println!("OK");
            } else {
                println!("NA");
            }
        }
    }
}

use std::io::{self, Read};

fn main() {
    // Read entire stdin as whitespace-separated tokens
    let mut input = String::new();
    // Ignore errors from read_to_string; per SPEC input is assumed well-formed.
    let _ = io::stdin().read_to_string(&mut input);

    let mut tokens = input.split_whitespace();

    // Parse N. SPEC requires the first token to be N; if missing or malformed, behavior is unspecified.
    let n: i64 = match tokens.next().and_then(|s| s.parse::<i64>().ok()) {
        Some(x) => x,
        None => return, // Exit silently if N isn't present (keeps behavior simple and safe)
    };

    // If N <= 0, produce no per-triple output (SPEC).
    if n <= 0 {
        return;
    }

    // Process up to N triples. SPEC assumes there are at least 3*N integers following N.
    for _ in 0..n {
        // Parse a, b, c. If any are missing or malformed, stop processing (SPEC leaves such cases undefined;
        // we choose to stop to avoid panic).
        let a = match tokens.next().and_then(|s| s.parse::<i128>().ok()) {
            Some(v) => v,
            None => return,
        };
        let b = match tokens.next().and_then(|s| s.parse::<i128>().ok()) {
            Some(v) => v,
            None => return,
        };
        let c = match tokens.next().and_then(|s| s.parse::<i128>().ok()) {
            Some(v) => v,
            None => return,
        };

        // To avoid any accidental overflow in Rust debug mode, use i128 for squaring.
        // SPEC guarantees inputs are such that C signed-int arithmetic won't overflow; using a wider
        // type here preserves the intended logical result for all valid inputs.
        let max = a.max(b).max(c);

        // Sum of squares of the other two values can be computed as total - max^2,
        // which matches treating "the largest of the three as the hypotenuse".
        let aa = a * a;
        let bb = b * b;
        let cc = c * c;
        let total = aa + bb + cc;
        let max_sq = max * max;
        let others_sq = total - max_sq;

        if max_sq == others_sq {
            println!("YES");
        } else {
            println!("NO");
        }
    }
}

use std::io::{self, Read};
use std::process;

fn main() {
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        process::exit(1);
    }

    let bytes = input.as_bytes();
    let mut i: usize = 0;

    fn is_ws(b: u8) -> bool {
        matches!(b, b' ' | b'\n' | b'\r' | b'\t' | 0x0b | 0x0c)
    }

    // Skip leading whitespace
    while i < bytes.len() && is_ws(bytes[i]) {
        i += 1;
    }

    // Parse n
    if i >= bytes.len() || !bytes[i].is_ascii_digit() {
        process::exit(1);
    }
    let mut n: usize = 0;
    while i < bytes.len() && bytes[i].is_ascii_digit() {
        let d = (bytes[i] - b'0') as usize;
        n = match n.checked_mul(10).and_then(|v| v.checked_add(d)) {
            Some(v) => v,
            None => process::exit(1),
        };
        i += 1;
    }
    if n < 2 {
        process::exit(1);
    }

    // The sample indicates that after n, there are n strings (lines/tokens),
    // each consisting of 'T'/'F'. The logical operation is the AND over these
    // n boolean values, where each value is the XOR of the characters in its string:
    // - A string evaluates to true iff it contains an odd number of 'T' characters.
    // - Final output is 'T' iff all n strings evaluate to true; otherwise 'F'.
    //
    // This interpretation matches the observed sample: all 5 strings have even # of 'T' => false,
    // AND => false => 'F'.

    let mut all_true = true;

    for _ in 0..n {
        // Skip whitespace before the next token
        while i < bytes.len() && is_ws(bytes[i]) {
            i += 1;
        }
        if i >= bytes.len() {
            process::exit(1);
        }

        // Read one token: maximal run of non-whitespace
        let start = i;
        while i < bytes.len() && !is_ws(bytes[i]) {
            i += 1;
        }
        let tok = &bytes[start..i];
        if tok.is_empty() {
            process::exit(1);
        }

        // Evaluate token parity of 'T' (XOR fold)
        let mut parity = false;
        for &b in tok {
            match b {
                b'T' => parity = !parity,
                b'F' => {}
                _ => process::exit(1),
            }
        }

        all_true &= parity;
    }

    let out = if all_true { 'T' } else { 'F' };
    print!("{out}");
}

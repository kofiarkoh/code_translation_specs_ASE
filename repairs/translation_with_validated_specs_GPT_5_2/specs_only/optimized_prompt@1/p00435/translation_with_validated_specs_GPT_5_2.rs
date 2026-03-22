use std::io::{self, Read};

fn main() {
    // Fixed-size buffer constraint: at most 999 characters (as per specification).
    // We read a single whitespace-delimited token; no explicit error handling.
    let mut input = String::new();
    let _ = io::stdin().read_to_string(&mut input);

    let token = input.split_whitespace().next().unwrap_or("");

    // Emulate fixed buffer behavior: if oversized, behavior is undefined in the spec.
    // We choose the narrowest reasonable interpretation: do not handle it.
    // (We still proceed; this does not add explicit error handling.)
    let bytes = token.as_bytes();

    let mut out = Vec::with_capacity(bytes.len());
    for &b in bytes {
        // Apply a fixed arithmetic adjustment: shift 3 positions backward.
        // For 'A'..'Z', wrap-around is required. For other bytes, output is unspecified;
        // we still apply the same arithmetic adjustment in a byte-wise manner.
        let ob = if (b'A'..=b'Z').contains(&b) {
            let idx = b - b'A';
            let shifted = (idx + 26 - 3) % 26;
            b'A' + shifted
        } else {
            b.wrapping_sub(3)
        };
        out.push(ob);
    }

    // Output followed by a line terminator.
    // The "null-terminated string" postcondition is a C-level notion; Rust strings are not
    // null-terminated. We preserve observable I/O by printing the transformed bytes.
    let _ = io::stdout().write_all(&out);
    let _ = io::stdout().write_all(b"\n");
}

use std::io::Write;

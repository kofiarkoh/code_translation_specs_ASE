use std::io::{self, Read};

fn main() {
    // Read all stdin bytes. The specification describes a null-terminated input string
    // with max length 999 excluding the terminator. In typical stdin, we may not receive
    // an explicit NUL; to preserve the "null-terminated" semantics, we process up to the
    // first NUL if present, otherwise process all bytes read.
    let mut input = Vec::new();
    io::stdin().read_to_end(&mut input).unwrap();

    // Determine the effective string length: up to (but not including) the first NUL.
    let end = input.iter().position(|&b| b == 0).unwrap_or(input.len());

    // Transform each byte according to ASCII value rules.
    // - Uppercase letters ('A'..='Z') shifted backward by 3.
    // - All other characters shifted forward by 23.
    // Shifts are performed with 8-bit wraparound to emulate byte/char arithmetic.
    let mut output = Vec::with_capacity(end + 1);
    for &b in &input[..end] {
        let transformed = if (b'A'..=b'Z').contains(&b) {
            b.wrapping_sub(3)
        } else {
            b.wrapping_add(23)
        };
        output.push(transformed);
    }

    // Ensure null-termination as per postconditions/invariants.
    output.push(0);

    // Write the resulting null-terminated byte string to stdout.
    // This preserves the observable behavior of emitting the transformed bytes.
    io::stdout().write_all(&output).unwrap();
}

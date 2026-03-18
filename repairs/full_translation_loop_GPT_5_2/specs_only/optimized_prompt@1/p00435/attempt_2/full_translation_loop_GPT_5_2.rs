use std::io::{self, Read, Write};

fn main() {
    // Read all stdin bytes. The specification describes a null-terminated input string
    // with max length 999 excluding the terminator. If a NUL is present, treat it as the
    // terminator; otherwise, process all bytes read.
    let mut input = Vec::new();
    io::stdin().read_to_end(&mut input).unwrap();

    // Effective length is up to (but not including) the first NUL byte.
    let end = input.iter().position(|&b| b == 0).unwrap_or(input.len());

    // Transform each byte:
    // - 'A'..='Z' => shift backward by 3
    // - otherwise => shift forward by 23
    // Use wrapping arithmetic to emulate 8-bit char behavior.
    let mut output = Vec::with_capacity(end + 1);
    for &b in &input[..end] {
        let transformed = if (b'A'..=b'Z').contains(&b) {
            b.wrapping_sub(3)
        } else {
            b.wrapping_add(23)
        };
        output.push(transformed);
    }

    // Ensure null-termination.
    output.push(0);

    // Write to stdout.
    io::stdout().write_all(&output).unwrap();
}

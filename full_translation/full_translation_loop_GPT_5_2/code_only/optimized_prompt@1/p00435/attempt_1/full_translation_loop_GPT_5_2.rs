use std::io::{self, Read};

fn main() {
    // Read entire stdin, then mimic `scanf("%s", a)` by taking the first
    // whitespace-delimited token (or empty if none).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let token = input.split_whitespace().next().unwrap_or("");

    // C uses `char a[1000]` and reads a C string. We'll operate on bytes.
    // The C code assumes uppercase ASCII letters; we preserve byte-wise behavior.
    let mut bytes = token.as_bytes().to_vec();

    for b in &mut bytes {
        if b'D' <= *b && *b <= b'Z' {
            *b = b.wrapping_sub(3);
        } else {
            *b = b.wrapping_add(23);
        }
    }

    // Print as bytes. This matches C's `%s\n` for typical ASCII inputs.
    // If the transformed bytes are not valid UTF-8, we still emit them lossily
    // rather than panicking, to keep output behavior robust.
    let out = String::from_utf8_lossy(&bytes);
    print!("{}\n", out);
}

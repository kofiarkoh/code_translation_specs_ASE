use std::io::{self, Read};

fn main() {
    // Read entire stdin, then take the first whitespace-delimited token.
    // Specification: if input reading fails, output is not inferable; we do no explicit handling.
    let mut input = String::new();
    let _ = io::stdin().read_to_string(&mut input);

    let token = match input.split_whitespace().next() {
        Some(t) => t,
        None => return, // No token; unspecified behavior, so produce no output.
    };

    // Specification implies a fixed-size buffer (< 1000) in the original.
    // In Rust we won't overflow memory, but we keep behavior narrow by not adding extra handling.
    // If token length >= 1000, the spec says behavior is undefined; we do not emulate UB.
    let mut out = String::with_capacity(token.len());

    for &b in token.as_bytes() {
        // Apply transformation per character, independently.
        // For 'D'..='Z': shift back by 3.
        // Otherwise: shift forward by 23 in the character encoding.
        // Use wrapping arithmetic to emulate byte-sized overflow behavior.
        let transformed = if (b'D'..=b'Z').contains(&b) {
            b.wrapping_sub(3)
        } else {
            b.wrapping_add(23)
        };
        out.push(transformed as char);
    }

    println!("{out}");
}

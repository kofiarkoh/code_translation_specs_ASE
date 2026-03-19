use std::io::{self, Read};

fn main() {
    // Read entire stdin; if parsing fails or inputs are missing, behavior is unspecified by the spec.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).ok();

    let mut it = input.split_whitespace();

    let n: i32 = match it.next().and_then(|s| s.parse::<i32>().ok()) {
        Some(v) => v,
        None => return, // No explicit error handling per spec.
    };

    // Spec: if initial count <= 0, produce no output.
    if n <= 0 {
        return;
    }

    let mut out = String::new();

    for _ in 0..n {
        let x: i32 = match it.next().and_then(|s| s.parse::<i32>().ok()) {
            Some(v) => v,
            None => break, // Unspecified; we stop producing further outputs.
        };
        let y: i32 = match it.next().and_then(|s| s.parse::<i32>().ok()) {
            Some(v) => v,
            None => break,
        };

        // Preconditions: y != 0 and no overflow. We do not add checks to preserve "undefined" behavior.
        // Note: This uses Rust's remainder semantics (truncating division), matching C for i32.
        let r = (x - 1) % y + 1;

        out.push_str(&r.to_string());
        out.push('\n');
    }

    print!("{out}");
}

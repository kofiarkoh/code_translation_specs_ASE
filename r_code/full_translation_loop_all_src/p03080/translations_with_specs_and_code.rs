use std::io::{self, Read};
use std::process;

fn main() {
    // Read entire stdin (matches typical competitive-programming style input).
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        process::exit(1);
    }
    let mut it = input.split_whitespace();

    let n: usize = match it.next().and_then(|t| t.parse::<usize>().ok()) {
        Some(v) => v,
        None => process::exit(1), // Spec: terminate non-zero if integer cannot be read
    };

    let s = match it.next() {
        Some(v) => v,
        None => process::exit(1), // Spec: terminate non-zero if string cannot be read
    };

    // Spec precondition: n <= s.len(). We assume valid input; if violated, C would read out of bounds.
    // We avoid unsafe; if violated, we treat missing chars as absent by limiting to available bytes.
    let bytes = s.as_bytes();
    let limit = n.min(bytes.len());

    let mut kitaeri: i32 = 0;
    for &b in &bytes[..limit] {
        if b == b'R' {
            kitaeri += 1;
        } else {
            kitaeri -= 1;
        }
    }

    // If n > s.len(), C has undefined behavior; spec says it won't happen.
    // We keep behavior well-defined without contradicting the spec's valid-input assumption.
    if kitaeri > 0 {
        println!("Yes");
    } else {
        println!("No");
    }
}

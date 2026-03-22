use std::io::{self, Read};

fn main() {
    // Read entire stdin (no explicit error handling, matching the C program's behavior expectations).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input.split_whitespace();

    // Process exactly 7 pairs.
    for _ in 0..7 {
        let a: i32 = it.next().unwrap().parse().unwrap();
        let b: i32 = it.next().unwrap().parse().unwrap();

        // C has undefined behavior on signed overflow; Rust would panic in debug.
        // Use wrapping_sub to avoid introducing a panic and keep behavior unspecified on overflow.
        let diff = a.wrapping_sub(b);
        println!("{diff}");
    }
}

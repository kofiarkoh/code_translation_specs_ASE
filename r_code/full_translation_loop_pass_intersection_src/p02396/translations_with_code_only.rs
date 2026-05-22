use std::io::{self, Read};

fn main() {
    // Read entire stdin; this matches typical competitive-programming style input.
    // Note: The original C code does not check scanf's return value; if input ends
    // unexpectedly, behavior is undefined. Here we stop when no more tokens exist.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut iter = input.split_whitespace();

    let mut i: i32 = 0;
    loop {
        let tok = match iter.next() {
            Some(t) => t,
            None => break, // best-effort when input is missing
        };

        // C's scanf("%d") parses into int; assume i32 here.
        let x: i32 = tok.parse().unwrap();

        // In C, signed overflow is UB; we use wrapping_add to avoid panics and
        // approximate typical two's-complement wraparound behavior.
        i = i.wrapping_add(1);

        if x == 0 {
            break;
        }
        print!("Case {}: {}\n", i, x);
    }
}

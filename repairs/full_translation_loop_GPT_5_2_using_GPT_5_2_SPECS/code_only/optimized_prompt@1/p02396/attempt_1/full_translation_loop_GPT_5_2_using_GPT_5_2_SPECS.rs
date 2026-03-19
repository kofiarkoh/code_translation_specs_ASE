use std::io::{self, Read};

fn main() {
    // Read entire stdin; this matches typical scanf-driven behavior for whitespace-separated ints.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut iter = input.split_whitespace();

    let mut i: i32 = 0;
    loop {
        // The C code does not check scanf's return value; if input ends early,
        // behavior is undefined. Here we stop if no more tokens are available.
        let x: i32 = match iter.next() {
            Some(tok) => tok.parse::<i32>().unwrap(),
            None => break,
        };

        // In C, signed overflow is undefined; we use wrapping_add to emulate
        // common two's-complement wraparound behavior.
        i = i.wrapping_add(1);

        if x == 0 {
            break;
        }
        print!("Case {}: {}\n", i, x);
    }
}

use std::io::{self, Read};

fn main() {
    // Read entire stdin (no explicit error handling per spec).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input.split_whitespace();

    for _ in 0..7 {
        let a: i32 = it.next().unwrap().parse().unwrap();
        let b: i32 = it.next().unwrap().parse().unwrap();

        // C has undefined behavior on signed overflow; Rust would panic in debug.
        // The spec assumes no overflow/underflow occurs, so normal subtraction is used.
        println!("{}", a - b);
    }
}

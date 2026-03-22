use std::io::{self, Read};

fn main() {
    // Read entire stdin, parse three integers: N, A, B (N is unused per spec).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let _n: i32 = it.next().unwrap().parse().unwrap();
    let a: i32 = it.next().unwrap().parse().unwrap();
    let b: i32 = it.next().unwrap().parse().unwrap();

    // Match C semantics for potential overflow in (B - A):
    // In C, signed overflow is undefined/unspecified; in Rust, we avoid panics by wrapping.
    let diff = b.wrapping_sub(a);

    if diff % 2 == 0 {
        print!("Alice\n");
    } else {
        print!("Borys\n");
    }
}

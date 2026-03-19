use std::io::{self, Read};

fn main() {
    // Read entire stdin, like scanf would consume from stdin.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input.split_whitespace();
    let a: i64 = match it.next() {
        Some(v) => v.parse().unwrap(),
        None => return, // If no input, C scanf would fail; just exit.
    };
    let b: i64 = match it.next() {
        Some(v) => v.parse().unwrap(),
        None => return,
    };

    // C uses `long int` and performs signed overflow with undefined behavior.
    // To emulate typical two's-complement wraparound behavior, use wrapping ops.
    let mut i: i64 = 1;
    // Loop condition: i <= b + 1 (with potential overflow in b+1 in C).
    while i <= b.wrapping_add(1) {
        let prod = a.wrapping_mul(i);
        // In C, `%` with b==0 is undefined (division by zero). We avoid panicking
        // and simply do nothing in that case, matching "no defined output".
        if b != 0 && prod.wrapping_rem(b) == 0 {
            print!("{}\n", prod);
            break;
        }
        i = i.wrapping_add(1);
    }
}

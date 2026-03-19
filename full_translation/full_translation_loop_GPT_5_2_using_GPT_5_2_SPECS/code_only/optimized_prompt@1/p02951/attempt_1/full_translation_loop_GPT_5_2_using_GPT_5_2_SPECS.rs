use std::io::{self, Read};

fn main() {
    // Read entire stdin, like scanf would consume from stdin.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    // The C code does not check scanf's return value; if input is missing,
    // behavior would be undefined. Here we assume the three ints are present.
    let mut a: i32 = it.next().unwrap().parse().unwrap();
    let b: i32 = it.next().unwrap().parse().unwrap();
    let mut c: i32 = it.next().unwrap().parse().unwrap();

    // In C, signed overflow is undefined. We use wrapping ops to emulate
    // typical two's-complement behavior as closely as possible.
    a = a.wrapping_sub(b);

    while a > 0 {
        a = a.wrapping_sub(1);
        c = c.wrapping_sub(1);
        if c == 0 {
            break;
        }
    }

    print!("{}\n", c);
}

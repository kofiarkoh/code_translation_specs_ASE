use std::io::{self, Read};

fn main() {
    // Read entire stdin (no explicit error handling per spec).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input.split_whitespace();
    let a: i32 = it.next().unwrap().parse().unwrap();
    let b: i32 = it.next().unwrap().parse().unwrap();

    // Matches the C code semantics:
    // - Integer division truncates toward zero.
    // - If there is a non-zero remainder, increment the quotient.
    // Note: Per spec, b is assumed positive; if b <= 0, behavior may not match mathematical ceiling.
    let mut c = a / b;
    if a % b != 0 {
        c += 1;
    }

    print!("{c}\n");
}

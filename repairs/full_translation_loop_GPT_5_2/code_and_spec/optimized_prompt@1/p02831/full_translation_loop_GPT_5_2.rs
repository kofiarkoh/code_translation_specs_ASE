use std::io::{self, Read};

fn main() {
    // Read entire stdin, parse two long integers.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let a: i64 = match it.next() {
        Some(s) => s.parse().unwrap(),
        None => return,
    };
    let b: i64 = match it.next() {
        Some(s) => s.parse().unwrap(),
        None => return,
    };

    // SPEC preconditions: a is non-negative, b is positive.
    // Note: SPEC mentions b==0 may be undefined; the C code would also be undefined (mod by zero).
    // We preserve that behavior by not handling b==0 specially.

    // The C code searches i in [1, b+1] and prints the first a*i divisible by b.
    // If none found in that range, it produces no output.
    for i in 1..=b + 1 {
        let prod = a * i;
        if prod % b == 0 {
            print!("{prod}\n");
            break;
        }
    }
}

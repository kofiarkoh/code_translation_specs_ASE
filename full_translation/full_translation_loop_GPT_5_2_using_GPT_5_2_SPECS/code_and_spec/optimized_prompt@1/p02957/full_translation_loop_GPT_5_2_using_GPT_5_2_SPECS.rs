use std::io::{self, Read};

fn main() {
    // Read entire stdin, parse two signed integers.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let a: i64 = it.next().unwrap().parse().unwrap();
    let b: i64 = it.next().unwrap().parse().unwrap();

    let sum = a + b;

    if sum % 2 != 0 {
        print!("IMPOSSIBLE");
    } else {
        // Spec: output (a + b) / 2 in exact integer arithmetic.
        // Note: C code uses "%d" (int) while computing with long int; spec does not constrain width.
        print!("{}", sum / 2);
    }
}

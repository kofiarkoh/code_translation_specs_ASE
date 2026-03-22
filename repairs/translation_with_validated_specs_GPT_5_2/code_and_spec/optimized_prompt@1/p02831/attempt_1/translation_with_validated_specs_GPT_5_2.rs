use std::io::{self, Read};

fn main() {
    // Read entire stdin, parse two signed integers.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let a: i64 = it.next().unwrap().parse().unwrap();
    let b: i64 = it.next().unwrap().parse().unwrap();

    // Spec precondition: b != 0 and no overflow in (a*i) and (b+1).
    // We preserve C semantics: iterate i = 1..=b+1 (inclusive) and print first a*i divisible by b.
    for i in 1..=b + 1 {
        let candidate = a * i;
        if candidate % b == 0 {
            print!("{}\n", candidate);
            break;
        }
    }
}

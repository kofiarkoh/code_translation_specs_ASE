use std::io::{self, Read};

fn main() {
    // Read entire stdin, parse two integers.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input.split_whitespace();
    let a: i32 = it.next().unwrap().parse().unwrap();
    let p: i32 = it.next().unwrap().parse().unwrap();

    // Preserve C integer arithmetic semantics (i32 with truncating division toward zero).
    let result = ((a * 3) + p) / 2;

    print!("{result}\n");
}

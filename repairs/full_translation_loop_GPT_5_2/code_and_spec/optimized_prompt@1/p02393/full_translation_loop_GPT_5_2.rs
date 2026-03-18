use std::io::{self, Read};

fn main() {
    // Read entire stdin, expecting exactly three integers per spec.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input.split_whitespace();
    let a: i32 = it.next().unwrap().parse().unwrap();
    let b: i32 = it.next().unwrap().parse().unwrap();
    let c: i32 = it.next().unwrap().parse().unwrap();

    // Sort in non-decreasing order while preserving permutation property.
    let mut v = [a, b, c];
    v.sort();

    print!("{} {} {}\n", v[0], v[1], v[2]);
}

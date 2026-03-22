use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input.split_whitespace();

    // Per specification: exactly three integers are available and parsing succeeds.
    // If not, behavior is undefined; we allow panic via unwrap().
    let a: i64 = it.next().unwrap().parse().unwrap();
    let b: i64 = it.next().unwrap().parse().unwrap();
    let c: i64 = it.next().unwrap().parse().unwrap();

    let mut v = [a, b, c];
    v.sort_unstable();

    println!("{} {} {}", v[0], v[1], v[2]);
}

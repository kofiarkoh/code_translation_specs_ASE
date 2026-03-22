use std::io::{self, Read};

fn main() {
    // Read entire stdin, parse three integers.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let a: i32 = it.next().unwrap().parse().unwrap();
    let b: i32 = it.next().unwrap().parse().unwrap();
    let c: i32 = it.next().unwrap().parse().unwrap();

    if a <= c && c <= b {
        println!("Yes");
    } else {
        println!("No");
    }
}

use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input.split_whitespace();
    let a: i128 = it.next().unwrap().parse().unwrap();
    let b: i128 = it.next().unwrap().parse().unwrap();

    let sum = a + b;

    if sum % 2 == 0 {
        print!("{}", sum / 2);
    } else {
        print!("IMPOSSIBLE");
    }
}

use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let day: i64 = input.split_whitespace().next().unwrap().parse().unwrap();

    match day {
        25 => println!("Christmas"),
        24 => println!("Christmas Eve"),
        23 => println!("Christmas Eve Eve"),
        22 => println!("Christmas Eve Eve Eve"),
        _ => {}
    }
}
